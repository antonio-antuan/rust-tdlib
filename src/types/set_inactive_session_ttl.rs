use crate::errors::Result;
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

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetInactiveSessionTtlBuilder {
        let mut inner = SetInactiveSessionTtl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setInactiveSessionTtl".to_string();

        SetInactiveSessionTtlBuilder { inner }
    }

    pub fn inactive_session_ttl_days(&self) -> i32 {
        self.inactive_session_ttl_days
    }
}

#[doc(hidden)]
pub struct SetInactiveSessionTtlBuilder {
    inner: SetInactiveSessionTtl,
}

#[deprecated]
pub type RTDSetInactiveSessionTtlBuilder = SetInactiveSessionTtlBuilder;

impl SetInactiveSessionTtlBuilder {
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

impl AsRef<SetInactiveSessionTtl> for SetInactiveSessionTtlBuilder {
    fn as_ref(&self) -> &SetInactiveSessionTtl {
        &self.inner
    }
}
