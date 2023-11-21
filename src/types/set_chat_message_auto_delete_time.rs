use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the message auto-delete or self-destruct (for secret chats) time in a chat. Requires change_info administrator right in basic groups, supergroups and channels Message auto-delete time can't be changed in a chat with the current user (Saved Messages) and the chat 777000 (Telegram).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatMessageAutoDeleteTime {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// New time value, in seconds; unless the chat is secret, it must be from 0 up to 365 * 86400 and be divisible by 86400. If 0, then messages aren't deleted automatically

    #[serde(default)]
    message_auto_delete_time: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetChatMessageAutoDeleteTime {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetChatMessageAutoDeleteTime {}

impl SetChatMessageAutoDeleteTime {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetChatMessageAutoDeleteTimeBuilder {
        let mut inner = SetChatMessageAutoDeleteTime::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setChatMessageAutoDeleteTime".to_string();

        SetChatMessageAutoDeleteTimeBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_auto_delete_time(&self) -> i32 {
        self.message_auto_delete_time
    }
}

#[doc(hidden)]
pub struct SetChatMessageAutoDeleteTimeBuilder {
    inner: SetChatMessageAutoDeleteTime,
}

#[deprecated]
pub type RTDSetChatMessageAutoDeleteTimeBuilder = SetChatMessageAutoDeleteTimeBuilder;

impl SetChatMessageAutoDeleteTimeBuilder {
    pub fn build(&self) -> SetChatMessageAutoDeleteTime {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_auto_delete_time(&mut self, message_auto_delete_time: i32) -> &mut Self {
        self.inner.message_auto_delete_time = message_auto_delete_time;
        self
    }
}

impl AsRef<SetChatMessageAutoDeleteTime> for SetChatMessageAutoDeleteTime {
    fn as_ref(&self) -> &SetChatMessageAutoDeleteTime {
        self
    }
}

impl AsRef<SetChatMessageAutoDeleteTime> for SetChatMessageAutoDeleteTimeBuilder {
    fn as_ref(&self) -> &SetChatMessageAutoDeleteTime {
        &self.inner
    }
}
