use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Invites a bot to a chat (if it is not yet a member) and sends it the /start command. Bots can't be invited to a private chat other than the chat with the bot. Bots can't be invited to channels (although they can be added as admins) and secret chats. Returns the sent message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendBotStartMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the bot
    bot_user_id: i32,
    /// Identifier of the target chat
    chat_id: i64,
    /// A hidden parameter sent to the bot for deep linking purposes (https://core.telegram.org/bots#deep-linking)
    parameter: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SendBotStartMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SendBotStartMessage {}

impl SendBotStartMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSendBotStartMessageBuilder {
        let mut inner = SendBotStartMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "sendBotStartMessage".to_string();

        RTDSendBotStartMessageBuilder { inner }
    }

    pub fn bot_user_id(&self) -> i32 {
        self.bot_user_id
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn parameter(&self) -> &String {
        &self.parameter
    }
}

#[doc(hidden)]
pub struct RTDSendBotStartMessageBuilder {
    inner: SendBotStartMessage,
}

impl RTDSendBotStartMessageBuilder {
    pub fn build(&self) -> SendBotStartMessage {
        self.inner.clone()
    }

    pub fn bot_user_id(&mut self, bot_user_id: i32) -> &mut Self {
        self.inner.bot_user_id = bot_user_id;
        self
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn parameter<T: AsRef<str>>(&mut self, parameter: T) -> &mut Self {
        self.inner.parameter = parameter.as_ref().to_string();
        self
    }
}

impl AsRef<SendBotStartMessage> for SendBotStartMessage {
    fn as_ref(&self) -> &SendBotStartMessage {
        self
    }
}

impl AsRef<SendBotStartMessage> for RTDSendBotStartMessageBuilder {
    fn as_ref(&self) -> &SendBotStartMessage {
        &self.inner
    }
}
