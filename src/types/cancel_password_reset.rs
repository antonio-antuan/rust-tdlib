use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Cancels reset of 2-step verification password. The method can be called if passwordState.pending_reset_date > 0
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CancelPasswordReset {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CancelPasswordReset {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CancelPasswordReset {}

impl CancelPasswordReset {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CancelPasswordResetBuilder {
        let mut inner = CancelPasswordReset::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "cancelPasswordReset".to_string();

        CancelPasswordResetBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CancelPasswordResetBuilder {
    inner: CancelPasswordReset,
}

#[deprecated]
pub type RTDCancelPasswordResetBuilder = CancelPasswordResetBuilder;

impl CancelPasswordResetBuilder {
    pub fn build(&self) -> CancelPasswordReset {
        self.inner.clone()
    }
}

impl AsRef<CancelPasswordReset> for CancelPasswordReset {
    fn as_ref(&self) -> &CancelPasswordReset {
        self
    }
}

impl AsRef<CancelPasswordReset> for CancelPasswordResetBuilder {
    fn as_ref(&self) -> &CancelPasswordReset {
        &self.inner
    }
}
