use async_nats::jetstream::stream::Stream;
use async_nats::jetstream::{stream, Context};
use async_nats::subject::ToSubject;
use async_nats::{jetstream, Subject};
use bamboo_common_core::queueing::{IntoBytes, NotificationError};
use std::fmt::{Display, Formatter};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub async fn get_nats() -> Result<async_nats::Client, NotificationError> {
    let client = async_nats::connect(
        std::env::var("NATS_SERVER").map_err(|err| NotificationError::new(err.to_string()))?,
    )
    .await
    .map_err(|err| NotificationError::new(err.to_string()))?;

    Ok(client)
}

pub async fn publish<P: IntoBytes>(queue: Queue, payload: P) -> Result<(), NotificationError> {
    let client = get_nats().await?;

    client
        .publish(queue, payload.into_bytes()?)
        .await
        .map_err(|err| NotificationError::new(err.to_string()))?;
    client
        .flush()
        .await
        .map_err(|err| NotificationError::new(err.to_string()))
}

pub async fn get_jetstream() -> Result<Context, NotificationError> {
    let client = get_nats().await?;

    Ok(jetstream::new(client))
}

pub async fn get_once_stream() -> Result<Stream, NotificationError> {
    get_jetstream()
        .await?
        .create_stream(jetstream::stream::Config {
            name: "BAMBOO".to_string(),
            retention: stream::RetentionPolicy::WorkQueue,
            subjects: Queue::iter()
                .map(|queue| queue.to_subject().to_string())
                .collect::<Vec<String>>(),
            ..Default::default()
        })
        .await
        .map_err(|err| NotificationError::new(format!("Failed to create stream {err}")))
}

pub async fn publish_once<P: IntoBytes>(queue: Queue, payload: P) -> Result<(), NotificationError> {
    let jetstream = get_jetstream().await?;

    jetstream
        .publish(queue, payload.into_bytes()?)
        .await
        .map_err(|err| NotificationError::new(format!("Failed to publish {err}")))?
        .await
        .map_err(|err| NotificationError::new(format!("Failed to publish {err}")))
        .map(|_| ())
}

#[derive(EnumIter)]
pub enum Queue {
    Events,
    Mails,
}

impl Display for Queue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Queue::Events => f.write_str("bamboo.events"),
            Queue::Mails => f.write_str("bamboo.mails"),
        }
    }
}

impl ToSubject for Queue {
    fn to_subject(&self) -> Subject {
        Subject::from(self.to_string())
    }
}
