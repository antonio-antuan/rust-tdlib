use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Checks whether a username can be set for a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckChatUsername {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier; must be identifier of a supergroup chat, or a channel chat, or a private chat with self, or zero if the chat is being created

    #[serde(default)]
    chat_id: i64,
    /// Username to be checked

    #[serde(default)]
    username: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CheckChatUsername {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCheckChatUsernameResult for CheckChatUsername {}

impl RFunction for CheckChatUsername {}

impl CheckChatUsername {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CheckChatUsernameBuilder {
        let mut inner = CheckChatUsername::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "checkChatUsername".to_string();

        CheckChatUsernameBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn username(&self) -> &String {
        &self.username
    }
}

#[doc(hidden)]
pub struct CheckChatUsernameBuilder {
    inner: CheckChatUsername,
}

#[deprecated]
pub type RTDCheckChatUsernameBuilder = CheckChatUsernameBuilder;

impl CheckChatUsernameBuilder {
    pub fn build(&self) -> CheckChatUsername {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn username<T: AsRef<str>>(&mut self, username: T) -> &mut Self {
        self.inner.username = username.as_ref().to_string();
        self
    }
}

impl AsRef<CheckChatUsername> for CheckChatUsername {
    fn as_ref(&self) -> &CheckChatUsername {
        self
    }
}

impl AsRef<CheckChatUsername> for CheckChatUsernameBuilder {
    fn as_ref(&self) -> &CheckChatUsername {
        &self.inner
    }
}
