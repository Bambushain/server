use async_nats::{jetstream, Message};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub trait IntoBytes {
    fn into_bytes(self) -> Result<Bytes, NotificationError>;
}

pub trait FromMessage<T: Sized> {
    fn from_message(message: Message) -> Result<T, NotificationError>;
    fn from_jetstream_message(message: jetstream::Message) -> Result<T, NotificationError>;
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct NotificationError {
    message: String,
}

impl Debug for NotificationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl Display for NotificationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl Error for NotificationError {}

impl NotificationError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

#[macro_export]
macro_rules! impl_nats {
    ($type: ty) => {
        impl $crate::IntoBytes for $type {
            fn into_bytes(self) -> Result<bytes::Bytes, $crate::NotificationError> {
                let mut data = Vec::<u8>::new();

                ciborium::into_writer(&self, &mut data)
                    .map_err(|err| $crate::NotificationError::new(err.to_string()))?;

                Ok(bytes::Bytes::copy_from_slice(data.as_slice()))
            }
        }

        impl $crate::FromMessage<$type> for $type {
            fn from_message(
                message: async_nats::Message,
            ) -> Result<$type, $crate::NotificationError> {
                ciborium::from_reader(message.payload.iter().as_slice())
                    .map_err(|err| $crate::NotificationError::new(err.to_string()))
            }

            fn from_jetstream_message(
                message: async_nats::jetstream::Message,
            ) -> Result<$type, $crate::NotificationError> {
                ciborium::from_reader(message.payload.iter().as_slice())
                    .map_err(|err| $crate::NotificationError::new(err.to_string()))
            }
        }
    };
}
