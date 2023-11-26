use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a slot for chat boost
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatBoostSlot {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique identifier of the slot

    #[serde(default)]
    slot_id: i32,
    /// Identifier of the currently boosted chat; 0 if none

    #[serde(default)]
    currently_boosted_chat_id: i64,
    /// Point in time (Unix timestamp) when the chat was boosted; 0 if none

    #[serde(default)]
    start_date: i32,
    /// Point in time (Unix timestamp) when the boost will expire

    #[serde(default)]
    expiration_date: i32,
    /// Point in time (Unix timestamp) after which the boost can be used for another chat

    #[serde(default)]
    cooldown_until_date: i32,
}

impl RObject for ChatBoostSlot {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatBoostSlot {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatBoostSlotBuilder {
        let mut inner = ChatBoostSlot::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatBoostSlotBuilder { inner }
    }

    pub fn slot_id(&self) -> i32 {
        self.slot_id
    }

    pub fn currently_boosted_chat_id(&self) -> i64 {
        self.currently_boosted_chat_id
    }

    pub fn start_date(&self) -> i32 {
        self.start_date
    }

    pub fn expiration_date(&self) -> i32 {
        self.expiration_date
    }

    pub fn cooldown_until_date(&self) -> i32 {
        self.cooldown_until_date
    }
}

#[doc(hidden)]
pub struct ChatBoostSlotBuilder {
    inner: ChatBoostSlot,
}

#[deprecated]
pub type RTDChatBoostSlotBuilder = ChatBoostSlotBuilder;

impl ChatBoostSlotBuilder {
    pub fn build(&self) -> ChatBoostSlot {
        self.inner.clone()
    }

    pub fn slot_id(&mut self, slot_id: i32) -> &mut Self {
        self.inner.slot_id = slot_id;
        self
    }

    pub fn currently_boosted_chat_id(&mut self, currently_boosted_chat_id: i64) -> &mut Self {
        self.inner.currently_boosted_chat_id = currently_boosted_chat_id;
        self
    }

    pub fn start_date(&mut self, start_date: i32) -> &mut Self {
        self.inner.start_date = start_date;
        self
    }

    pub fn expiration_date(&mut self, expiration_date: i32) -> &mut Self {
        self.inner.expiration_date = expiration_date;
        self
    }

    pub fn cooldown_until_date(&mut self, cooldown_until_date: i32) -> &mut Self {
        self.inner.cooldown_until_date = cooldown_until_date;
        self
    }
}

impl AsRef<ChatBoostSlot> for ChatBoostSlot {
    fn as_ref(&self) -> &ChatBoostSlot {
        self
    }
}

impl AsRef<ChatBoostSlot> for ChatBoostSlotBuilder {
    fn as_ref(&self) -> &ChatBoostSlot {
        &self.inner
    }
}
