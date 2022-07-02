use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Deletes saved order info
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteSavedOrderInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DeleteSavedOrderInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DeleteSavedOrderInfo {}

impl DeleteSavedOrderInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DeleteSavedOrderInfoBuilder {
        let mut inner = DeleteSavedOrderInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "deleteSavedOrderInfo".to_string();

        DeleteSavedOrderInfoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct DeleteSavedOrderInfoBuilder {
    inner: DeleteSavedOrderInfo,
}

#[deprecated]
pub type RTDDeleteSavedOrderInfoBuilder = DeleteSavedOrderInfoBuilder;

impl DeleteSavedOrderInfoBuilder {
    pub fn build(&self) -> DeleteSavedOrderInfo {
        self.inner.clone()
    }
}

impl AsRef<DeleteSavedOrderInfo> for DeleteSavedOrderInfo {
    fn as_ref(&self) -> &DeleteSavedOrderInfo {
        self
    }
}

impl AsRef<DeleteSavedOrderInfo> for DeleteSavedOrderInfoBuilder {
    fn as_ref(&self) -> &DeleteSavedOrderInfo {
        &self.inner
    }
}
