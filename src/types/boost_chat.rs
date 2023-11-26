use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Boosts a chat and returns the list of available chat boost slots for the current user after the boost
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BoostChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat

    #[serde(default)]
    chat_id: i64,
    /// Identifiers of boost slots of the current user from which to apply boosts to the chat

    #[serde(default)]
    slot_ids: Vec<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for BoostChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for BoostChat {}

impl BoostChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> BoostChatBuilder {
        let mut inner = BoostChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "boostChat".to_string();

        BoostChatBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn slot_ids(&self) -> &Vec<i32> {
        &self.slot_ids
    }
}

#[doc(hidden)]
pub struct BoostChatBuilder {
    inner: BoostChat,
}

#[deprecated]
pub type RTDBoostChatBuilder = BoostChatBuilder;

impl BoostChatBuilder {
    pub fn build(&self) -> BoostChat {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn slot_ids(&mut self, slot_ids: Vec<i32>) -> &mut Self {
        self.inner.slot_ids = slot_ids;
        self
    }
}

impl AsRef<BoostChat> for BoostChat {
    fn as_ref(&self) -> &BoostChat {
        self
    }
}

impl AsRef<BoostChat> for BoostChatBuilder {
    fn as_ref(&self) -> &BoostChat {
        &self.inner
    }
}
