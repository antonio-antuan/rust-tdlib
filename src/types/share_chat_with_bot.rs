use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Shares a chat after pressing a keyboardButtonTypeRequestChat button with the bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ShareChatWithBot {
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
    /// Identifier of the shared chat

    #[serde(default)]
    shared_chat_id: i64,
    /// Pass true to check that the chat can be shared by the button instead of actually sharing it. Doesn't check bot_is_member and bot_administrator_rights restrictions. If the bot must be a member, then all chats from getGroupsInCommon and all chats, where the user can add the bot, are suitable. In the latter case the bot will be automatically added to the chat. If the bot must be an administrator, then all chats, where the bot already has requested rights or can be added to administrators by the user, are suitable. In the latter case the bot will be automatically granted requested rights

    #[serde(default)]
    only_check: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ShareChatWithBot {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ShareChatWithBot {}

impl ShareChatWithBot {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ShareChatWithBotBuilder {
        let mut inner = ShareChatWithBot::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "shareChatWithBot".to_string();

        ShareChatWithBotBuilder { inner }
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

    pub fn shared_chat_id(&self) -> i64 {
        self.shared_chat_id
    }

    pub fn only_check(&self) -> bool {
        self.only_check
    }
}

#[doc(hidden)]
pub struct ShareChatWithBotBuilder {
    inner: ShareChatWithBot,
}

#[deprecated]
pub type RTDShareChatWithBotBuilder = ShareChatWithBotBuilder;

impl ShareChatWithBotBuilder {
    pub fn build(&self) -> ShareChatWithBot {
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

    pub fn shared_chat_id(&mut self, shared_chat_id: i64) -> &mut Self {
        self.inner.shared_chat_id = shared_chat_id;
        self
    }

    pub fn only_check(&mut self, only_check: bool) -> &mut Self {
        self.inner.only_check = only_check;
        self
    }
}

impl AsRef<ShareChatWithBot> for ShareChatWithBot {
    fn as_ref(&self) -> &ShareChatWithBot {
        self
    }
}

impl AsRef<ShareChatWithBot> for ShareChatWithBotBuilder {
    fn as_ref(&self) -> &ShareChatWithBot {
        &self.inner
    }
}
