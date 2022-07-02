use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the message TTL in a chat. Requires can_delete_messages administrator right in basic groups, supergroups and channels Message TTL can't be changed in a chat with the current user (Saved Messages) and the chat 777000 (Telegram)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatMessageTtl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// New TTL value, in seconds; must be one of 0, 86400, 7 * 86400, or 31 * 86400 unless the chat is secret

    #[serde(default)]
    ttl: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetChatMessageTtl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetChatMessageTtl {}

impl SetChatMessageTtl {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetChatMessageTtlBuilder {
        let mut inner = SetChatMessageTtl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setChatMessageTtl".to_string();

        SetChatMessageTtlBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn ttl(&self) -> i32 {
        self.ttl
    }
}

#[doc(hidden)]
pub struct SetChatMessageTtlBuilder {
    inner: SetChatMessageTtl,
}

#[deprecated]
pub type RTDSetChatMessageTtlBuilder = SetChatMessageTtlBuilder;

impl SetChatMessageTtlBuilder {
    pub fn build(&self) -> SetChatMessageTtl {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn ttl(&mut self, ttl: i32) -> &mut Self {
        self.inner.ttl = ttl;
        self
    }
}

impl AsRef<SetChatMessageTtl> for SetChatMessageTtl {
    fn as_ref(&self) -> &SetChatMessageTtl {
        self
    }
}

impl AsRef<SetChatMessageTtl> for SetChatMessageTtlBuilder {
    fn as_ref(&self) -> &SetChatMessageTtl {
        &self.inner
    }
}
