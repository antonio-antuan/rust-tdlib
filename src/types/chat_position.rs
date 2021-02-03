use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a position of a chat in a chat list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatPosition {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The chat list
    list: ChatList,
    /// A parameter used to determine order of the chat in the chat list. Chats must be sorted by the pair (order, chat.id) in descending order
    #[serde(deserialize_with = "super::_common::number_from_string")]
    order: i64,
    /// True, if the chat is pinned in the chat list
    is_pinned: bool,
    /// Source of the chat in the chat list; may be null
    source: Option<ChatSource>,
}

impl RObject for ChatPosition {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "chatPosition"
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

impl ChatPosition {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatPositionBuilder {
        let mut inner = ChatPosition::default();
        inner.td_name = "chatPosition".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDChatPositionBuilder { inner }
    }

    pub fn list(&self) -> &ChatList {
        &self.list
    }

    pub fn order(&self) -> i64 {
        self.order
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }

    pub fn source(&self) -> &Option<ChatSource> {
        &self.source
    }
}

#[doc(hidden)]
pub struct RTDChatPositionBuilder {
    inner: ChatPosition,
}

impl RTDChatPositionBuilder {
    pub fn build(&self) -> ChatPosition {
        self.inner.clone()
    }

    pub fn list<T: AsRef<ChatList>>(&mut self, list: T) -> &mut Self {
        self.inner.list = list.as_ref().clone();
        self
    }

    pub fn order(&mut self, order: i64) -> &mut Self {
        self.inner.order = order;
        self
    }

    pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
        self.inner.is_pinned = is_pinned;
        self
    }

    pub fn source<T: AsRef<ChatSource>>(&mut self, source: T) -> &mut Self {
        self.inner.source = Some(source.as_ref().clone());
        self
    }
}

impl AsRef<ChatPosition> for ChatPosition {
    fn as_ref(&self) -> &ChatPosition {
        self
    }
}

impl AsRef<ChatPosition> for RTDChatPositionBuilder {
    fn as_ref(&self) -> &ChatPosition {
        &self.inner
    }
}
