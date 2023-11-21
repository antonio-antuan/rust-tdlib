use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns the list of available chat boost slots for the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAvailableChatBoostSlots {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetAvailableChatBoostSlots {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetAvailableChatBoostSlots {}

impl GetAvailableChatBoostSlots {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetAvailableChatBoostSlotsBuilder {
        let mut inner = GetAvailableChatBoostSlots::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getAvailableChatBoostSlots".to_string();

        GetAvailableChatBoostSlotsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetAvailableChatBoostSlotsBuilder {
    inner: GetAvailableChatBoostSlots,
}

#[deprecated]
pub type RTDGetAvailableChatBoostSlotsBuilder = GetAvailableChatBoostSlotsBuilder;

impl GetAvailableChatBoostSlotsBuilder {
    pub fn build(&self) -> GetAvailableChatBoostSlots {
        self.inner.clone()
    }
}

impl AsRef<GetAvailableChatBoostSlots> for GetAvailableChatBoostSlots {
    fn as_ref(&self) -> &GetAvailableChatBoostSlots {
        self
    }
}

impl AsRef<GetAvailableChatBoostSlots> for GetAvailableChatBoostSlotsBuilder {
    fn as_ref(&self) -> &GetAvailableChatBoostSlots {
        &self.inner
    }
}
