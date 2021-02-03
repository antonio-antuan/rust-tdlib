use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a chat located nearby
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatNearby {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
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
    fn td_name(&self) -> &'static str {
        "chatNearby"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl ChatNearby {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatNearbyBuilder {
        let mut inner = ChatNearby::default();
        inner.td_name = "chatNearby".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
