use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes the period of inactivity after which sessions will automatically be terminated
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetInactiveSessionTtl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// New number of days of inactivity before sessions will be automatically terminated; 1-366 days
    inactive_session_ttl_days: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetInactiveSessionTtl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetInactiveSessionTtl {}

impl SetInactiveSessionTtl {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetInactiveSessionTtlBuilder {
        let mut inner = SetInactiveSessionTtl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setInactiveSessionTtl".to_string();

        RTDSetInactiveSessionTtlBuilder { inner }
    }

    pub fn inactive_session_ttl_days(&self) -> i32 {
        self.inactive_session_ttl_days
    }
}

#[doc(hidden)]
pub struct RTDSetInactiveSessionTtlBuilder {
    inner: SetInactiveSessionTtl,
}

impl RTDSetInactiveSessionTtlBuilder {
    pub fn build(&self) -> SetInactiveSessionTtl {
        self.inner.clone()
    }

    pub fn inactive_session_ttl_days(&mut self, inactive_session_ttl_days: i32) -> &mut Self {
        self.inner.inactive_session_ttl_days = inactive_session_ttl_days;
        self
    }
}

impl AsRef<SetInactiveSessionTtl> for SetInactiveSessionTtl {
    fn as_ref(&self) -> &SetInactiveSessionTtl {
        self
    }
}

impl AsRef<SetInactiveSessionTtl> for RTDSetInactiveSessionTtlBuilder {
    fn as_ref(&self) -> &SetInactiveSessionTtl {
        &self.inner
    }
}
