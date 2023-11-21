use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Allows the specified bot to send messages to the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AllowBotToSendMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the target bot

    #[serde(default)]
    bot_user_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AllowBotToSendMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AllowBotToSendMessages {}

impl AllowBotToSendMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AllowBotToSendMessagesBuilder {
        let mut inner = AllowBotToSendMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "allowBotToSendMessages".to_string();

        AllowBotToSendMessagesBuilder { inner }
    }

    pub fn bot_user_id(&self) -> i64 {
        self.bot_user_id
    }
}

#[doc(hidden)]
pub struct AllowBotToSendMessagesBuilder {
    inner: AllowBotToSendMessages,
}

#[deprecated]
pub type RTDAllowBotToSendMessagesBuilder = AllowBotToSendMessagesBuilder;

impl AllowBotToSendMessagesBuilder {
    pub fn build(&self) -> AllowBotToSendMessages {
        self.inner.clone()
    }

    pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
        self.inner.bot_user_id = bot_user_id;
        self
    }
}

impl AsRef<AllowBotToSendMessages> for AllowBotToSendMessages {
    fn as_ref(&self) -> &AllowBotToSendMessages {
        self
    }
}

impl AsRef<AllowBotToSendMessages> for AllowBotToSendMessagesBuilder {
    fn as_ref(&self) -> &AllowBotToSendMessages {
        &self.inner
    }
}
