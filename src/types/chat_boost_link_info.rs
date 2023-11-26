use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a link to boost a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatBoostLinkInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if the link will work for non-members of the chat

    #[serde(default)]
    is_public: bool,
    /// Identifier of the chat to which the link points; 0 if the chat isn't found

    #[serde(default)]
    chat_id: i64,
}

impl RObject for ChatBoostLinkInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatBoostLinkInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatBoostLinkInfoBuilder {
        let mut inner = ChatBoostLinkInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatBoostLinkInfoBuilder { inner }
    }

    pub fn is_public(&self) -> bool {
        self.is_public
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct ChatBoostLinkInfoBuilder {
    inner: ChatBoostLinkInfo,
}

#[deprecated]
pub type RTDChatBoostLinkInfoBuilder = ChatBoostLinkInfoBuilder;

impl ChatBoostLinkInfoBuilder {
    pub fn build(&self) -> ChatBoostLinkInfo {
        self.inner.clone()
    }

    pub fn is_public(&mut self, is_public: bool) -> &mut Self {
        self.inner.is_public = is_public;
        self
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<ChatBoostLinkInfo> for ChatBoostLinkInfo {
    fn as_ref(&self) -> &ChatBoostLinkInfo {
        self
    }
}

impl AsRef<ChatBoostLinkInfo> for ChatBoostLinkInfoBuilder {
    fn as_ref(&self) -> &ChatBoostLinkInfo {
        &self.inner
    }
}
