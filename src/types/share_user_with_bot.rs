use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Shares a user after pressing a keyboardButtonTypeRequestUser button with the bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShareUserWithBot {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat with the bot

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the message with the button

    #[serde(default)]
    message_id: i64,
    /// Identifier of the button

    #[serde(default)]
    button_id: i32,
    /// Identifier of the shared user

    #[serde(default)]
    shared_user_id: i64,
    /// Pass true to check that the user can be shared by the button instead of actually sharing them

    #[serde(default)]
    only_check: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ShareUserWithBot {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ShareUserWithBot {}

impl ShareUserWithBot {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ShareUserWithBotBuilder {
        let mut inner = ShareUserWithBot::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "shareUserWithBot".to_string();

        ShareUserWithBotBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn button_id(&self) -> i32 {
        self.button_id
    }

    pub fn shared_user_id(&self) -> i64 {
        self.shared_user_id
    }

    pub fn only_check(&self) -> bool {
        self.only_check
    }
}

#[doc(hidden)]
pub struct ShareUserWithBotBuilder {
    inner: ShareUserWithBot,
}

#[deprecated]
pub type RTDShareUserWithBotBuilder = ShareUserWithBotBuilder;

impl ShareUserWithBotBuilder {
    pub fn build(&self) -> ShareUserWithBot {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn button_id(&mut self, button_id: i32) -> &mut Self {
        self.inner.button_id = button_id;
        self
    }

    pub fn shared_user_id(&mut self, shared_user_id: i64) -> &mut Self {
        self.inner.shared_user_id = shared_user_id;
        self
    }

    pub fn only_check(&mut self, only_check: bool) -> &mut Self {
        self.inner.only_check = only_check;
        self
    }
}

impl AsRef<ShareUserWithBot> for ShareUserWithBot {
    fn as_ref(&self) -> &ShareUserWithBot {
        self
    }
}

impl AsRef<ShareUserWithBot> for ShareUserWithBotBuilder {
    fn as_ref(&self) -> &ShareUserWithBot {
        &self.inner
    }
}
