use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of chat boost slots
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatBoostSlots {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of boost slots

    #[serde(default)]
    slots: Vec<ChatBoostSlot>,
}

impl RObject for ChatBoostSlots {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatBoostSlots {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatBoostSlotsBuilder {
        let mut inner = ChatBoostSlots::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatBoostSlotsBuilder { inner }
    }

    pub fn slots(&self) -> &Vec<ChatBoostSlot> {
        &self.slots
    }
}

#[doc(hidden)]
pub struct ChatBoostSlotsBuilder {
    inner: ChatBoostSlots,
}

#[deprecated]
pub type RTDChatBoostSlotsBuilder = ChatBoostSlotsBuilder;

impl ChatBoostSlotsBuilder {
    pub fn build(&self) -> ChatBoostSlots {
        self.inner.clone()
    }

    pub fn slots(&mut self, slots: Vec<ChatBoostSlot>) -> &mut Self {
        self.inner.slots = slots;
        self
    }
}

impl AsRef<ChatBoostSlots> for ChatBoostSlots {
    fn as_ref(&self) -> &ChatBoostSlots {
        self
    }
}

impl AsRef<ChatBoostSlots> for ChatBoostSlotsBuilder {
    fn as_ref(&self) -> &ChatBoostSlots {
        &self.inner
    }
}
