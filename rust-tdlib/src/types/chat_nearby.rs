use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a chat located nearby
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatNearby {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// Distance to the chat location, in meters
    distance: i32,
}

impl RObject for ChatNearby {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatNearby {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatNearbyBuilder {
        let mut inner = ChatNearby::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatNearbyBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn distance(&self) -> i32 {
        self.distance
    }
}

#[doc(hidden)]
pub struct RTDChatNearbyBuilder {
    inner: ChatNearby,
}

impl RTDChatNearbyBuilder {
    pub fn build(&self) -> ChatNearby {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn distance(&mut self, distance: i32) -> &mut Self {
        self.inner.distance = distance;
        self
    }
}

impl AsRef<ChatNearby> for ChatNearby {
    fn as_ref(&self) -> &ChatNearby {
        self
    }
}

impl AsRef<ChatNearby> for RTDChatNearbyBuilder {
    fn as_ref(&self) -> &ChatNearby {
        &self.inner
    }
}
