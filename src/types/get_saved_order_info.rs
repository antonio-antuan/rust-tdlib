use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns saved order info, if any
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSavedOrderInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetSavedOrderInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetSavedOrderInfo {}

impl GetSavedOrderInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetSavedOrderInfoBuilder {
        let mut inner = GetSavedOrderInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getSavedOrderInfo".to_string();

        GetSavedOrderInfoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetSavedOrderInfoBuilder {
    inner: GetSavedOrderInfo,
}

#[deprecated]
pub type RTDGetSavedOrderInfoBuilder = GetSavedOrderInfoBuilder;

impl GetSavedOrderInfoBuilder {
    pub fn build(&self) -> GetSavedOrderInfo {
        self.inner.clone()
    }
}

impl AsRef<GetSavedOrderInfo> for GetSavedOrderInfo {
    fn as_ref(&self) -> &GetSavedOrderInfo {
        self
    }
}

impl AsRef<GetSavedOrderInfo> for GetSavedOrderInfoBuilder {
    fn as_ref(&self) -> &GetSavedOrderInfo {
        &self.inner
    }
}
