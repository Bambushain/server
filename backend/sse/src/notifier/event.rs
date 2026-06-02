use std::sync::Arc;
use std::time::Duration;

use actix_web::rt::time::interval;
use actix_web_lab::sse;
use actix_web_lab::util::InfallibleStream;
use parking_lot::Mutex;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::Sender;

use bamboo_common::core::entities::user::BambooUser;
use bamboo_common::core::entities::{Grove, GroveEvent};
use bamboo_common::core::queueing::{EventAction, EventType};
use serde::{Deserialize, Serialize};
use tokio_stream::wrappers::ReceiverStream;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Event {
    event: GroveEvent,
    action: EventType,
}

impl From<Event> for sse::Event {
    fn from(event: Event) -> Self {
        let mut data = sse::Data::new_json(event.event.clone()).unwrap();
        data.set_event(event.action.to_string());

        sse::Event::Data(data)
    }
}

impl Event {
    fn new(event: GroveEvent, action: EventType) -> Self {
        Self { event, action }
    }

    fn from_event_action(event_action: EventAction) -> Self {
        match event_action {
            EventAction::Created(evt) => Event::new(evt, EventType::Created),
            EventAction::Updated(evt) => Event::new(evt, EventType::Updated),
            EventAction::Deleted(evt) => Event::new(evt, EventType::Deleted),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Comment {
    Connected,
    Ping,
}

pub struct EventBroadcaster {
    inner: Mutex<EventBroadcasterInner>,
}

#[derive(Debug, Clone, Default)]
struct EventBroadcasterInner {
    clients: Vec<(Sender<sse::Event>, BambooUser)>,
}

impl EventBroadcaster {
    pub fn create() -> Arc<Self> {
        let this = Arc::new(EventBroadcaster {
            inner: Mutex::new(EventBroadcasterInner::default()),
        });
        EventBroadcaster::spawn_ping(Arc::clone(&this));

        this
    }

    fn spawn_ping(this: Arc<Self>) {
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));

            loop {
                interval.tick().await;
                this.remove_stale_clients().await;
            }
        });
    }

    async fn remove_stale_clients(&self) {
        let clients = self.inner.lock().clients.clone();
        let mut ok_clients = Vec::new();
        for (client, user) in clients {
            if let Err(err) = Self::send_comment(client.clone(), Comment::Ping).await {
                log::info!("Failed to send ping {err}");
            } else {
                ok_clients.push((client.clone(), user));
            }
        }

        self.inner.lock().clients = ok_clients;
    }

    pub async fn new_client(
        &self,
        user: BambooUser,
    ) -> sse::Sse<InfallibleStream<ReceiverStream<sse::Event>>> {
        log::debug!("Open channel using tokio");
        let (tx, rx) = tokio::sync::mpsc::channel::<sse::Event>(10);

        log::debug!("Send connected message");
        if let Err(err) = Self::send_comment(tx.clone(), Comment::Connected).await {
            log::error!("Failed to send connect comment {err}")
        }
        self.inner.lock().clients.push((tx, user));

        sse::Sse::from_infallible_receiver(rx)
    }

    pub async fn send_event(&self, evt: EventAction, groves: Vec<Grove>) {
        let clients = self.inner.lock().clients.clone();
        log::debug!("Has {} clients registered", clients.len());
        let send_futures = clients.iter().filter_map(|(client, user)| {
            if Self::check_for_grove(user.clone(), evt.clone(), groves.clone()) {
                Some(client.send(Event::from_event_action(evt.clone()).into()))
            } else {
                None
            }
        });
        let res = futures_util::future::join_all(send_futures).await;
        for res in res {
            if let Err(err) = res {
                log::error!("Failed to send message {err}")
            }
        }
    }

    fn check_for_grove(user: BambooUser, evt: EventAction, groves: Vec<Grove>) -> bool {
        let event = match evt.clone() {
            EventAction::Created(event) => event,
            EventAction::Updated(event) => event,
            EventAction::Deleted(event) => event,
        };

        let is_private_event_of_current_user =
            event.is_private && Some(user.id) == event.user.map(|u| u.id);
        let is_in_same_grove = !event.is_private
            && groves
            .iter()
            .any(|g| g.id == event.grove.clone().map(|g| g.id).unwrap_or(-1));

        is_private_event_of_current_user || is_in_same_grove
    }

    async fn send_comment(
        client: Sender<sse::Event>,
        evt: Comment,
    ) -> Result<(), SendError<sse::Event>> {
        client
            .send(sse::Event::Comment(bytestring::ByteString::from(
                serde_json::to_string(&evt).unwrap(),
            )))
            .await
    }
}
