use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Checks whether the specified bot can send messages to the user. Returns a 404 error if can't and the access can be granted by call to allowBotToSendMessages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanBotSendMessages {
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

impl RObject for CanBotSendMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CanBotSendMessages {}

impl CanBotSendMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CanBotSendMessagesBuilder {
        let mut inner = CanBotSendMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "canBotSendMessages".to_string();

        CanBotSendMessagesBuilder { inner }
    }

    pub fn bot_user_id(&self) -> i64 {
        self.bot_user_id
    }
}

#[doc(hidden)]
pub struct CanBotSendMessagesBuilder {
    inner: CanBotSendMessages,
}

#[deprecated]
pub type RTDCanBotSendMessagesBuilder = CanBotSendMessagesBuilder;

impl CanBotSendMessagesBuilder {
    pub fn build(&self) -> CanBotSendMessages {
        self.inner.clone()
    }

    pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
        self.inner.bot_user_id = bot_user_id;
        self
    }
}

impl AsRef<CanBotSendMessages> for CanBotSendMessages {
    fn as_ref(&self) -> &CanBotSendMessages {
        self
    }
}

impl AsRef<CanBotSendMessages> for CanBotSendMessagesBuilder {
    fn as_ref(&self) -> &CanBotSendMessages {
        &self.inner
    }
}
