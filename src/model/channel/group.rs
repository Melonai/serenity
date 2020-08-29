use chrono::{DateTime, FixedOffset};
use crate::model::prelude::*;

#[cfg(feature = "model")]
use std::borrow::Cow;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Group {
    #[serde(rename = "id")]
    pub channel_id: ChannelId,
    pub icon: Option<String>,
    pub last_message_id: Option<MessageId>,
    pub last_pin_timestamp: Option<DateTime<FixedOffset>>,
    pub name: Option<String>,
    pub owner_id: UserId,
    #[serde(skip)]
    pub(crate) _nonexhaustive: (),
}

#[cfg(feature = "model")]
impl Group {
    pub fn name(&self) -> Cow<'_, str> {
        match self.name {
            Some(ref name) => Cow::Borrowed(name),
            None => Cow::Owned("Group DM".to_string())
        }
    }
}