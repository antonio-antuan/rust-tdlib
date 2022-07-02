use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Closes the TDLib instance, destroying all local data without a proper logout. The current user session will remain in the list of all active sessions. All local data will be destroyed. After the destruction completes updateAuthorizationState with authorizationStateClosed will be sent. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Destroy {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for Destroy {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for Destroy {}

impl Destroy {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DestroyBuilder {
        let mut inner = Destroy::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "destroy".to_string();

        DestroyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct DestroyBuilder {
    inner: Destroy,
}

#[deprecated]
pub type RTDDestroyBuilder = DestroyBuilder;

impl DestroyBuilder {
    pub fn build(&self) -> Destroy {
        self.inner.clone()
    }
}

impl AsRef<Destroy> for Destroy {
    fn as_ref(&self) -> &Destroy {
        self
    }
}

impl AsRef<Destroy> for DestroyBuilder {
    fn as_ref(&self) -> &Destroy {
        &self.inner
    }
}
