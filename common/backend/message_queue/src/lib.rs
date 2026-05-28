use async_nats::subject::ToSubject;
use async_nats::Subject;
use bamboo_common_core::queueing::{IntoBytes, NotificationError};
use std::fmt::{Display, Formatter};
use strum::EnumIter;

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

#[derive(EnumIter)]
pub enum Queue {
    Events,
}

impl Display for Queue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Queue::Events => f.write_str("bamboo.events"),
        }
    }
}

impl ToSubject for Queue {
    fn to_subject(&self) -> Subject {
        Subject::from(self.to_string())
    }
}
