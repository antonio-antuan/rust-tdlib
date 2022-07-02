use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the slow mode delay of a chat. Available only for supergroups; requires can_restrict_members rights
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatSlowModeDelay {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// New slow mode delay for the chat, in seconds; must be one of 0, 10, 30, 60, 300, 900, 3600

    #[serde(default)]
    slow_mode_delay: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetChatSlowModeDelay {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetChatSlowModeDelay {}

impl SetChatSlowModeDelay {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetChatSlowModeDelayBuilder {
        let mut inner = SetChatSlowModeDelay::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setChatSlowModeDelay".to_string();

        SetChatSlowModeDelayBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn slow_mode_delay(&self) -> i32 {
        self.slow_mode_delay
    }
}

#[doc(hidden)]
pub struct SetChatSlowModeDelayBuilder {
    inner: SetChatSlowModeDelay,
}

#[deprecated]
pub type RTDSetChatSlowModeDelayBuilder = SetChatSlowModeDelayBuilder;

impl SetChatSlowModeDelayBuilder {
    pub fn build(&self) -> SetChatSlowModeDelay {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn slow_mode_delay(&mut self, slow_mode_delay: i32) -> &mut Self {
        self.inner.slow_mode_delay = slow_mode_delay;
        self
    }
}

impl AsRef<SetChatSlowModeDelay> for SetChatSlowModeDelay {
    fn as_ref(&self) -> &SetChatSlowModeDelay {
        self
    }
}

impl AsRef<SetChatSlowModeDelay> for SetChatSlowModeDelayBuilder {
    fn as_ref(&self) -> &SetChatSlowModeDelay {
        &self.inner
    }
}
