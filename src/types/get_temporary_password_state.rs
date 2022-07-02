use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about the current temporary password
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetTemporaryPasswordState {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetTemporaryPasswordState {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetTemporaryPasswordState {}

impl GetTemporaryPasswordState {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetTemporaryPasswordStateBuilder {
        let mut inner = GetTemporaryPasswordState::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getTemporaryPasswordState".to_string();

        GetTemporaryPasswordStateBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetTemporaryPasswordStateBuilder {
    inner: GetTemporaryPasswordState,
}

#[deprecated]
pub type RTDGetTemporaryPasswordStateBuilder = GetTemporaryPasswordStateBuilder;

impl GetTemporaryPasswordStateBuilder {
    pub fn build(&self) -> GetTemporaryPasswordState {
        self.inner.clone()
    }
}

impl AsRef<GetTemporaryPasswordState> for GetTemporaryPasswordState {
    fn as_ref(&self) -> &GetTemporaryPasswordState {
        self
    }
}

impl AsRef<GetTemporaryPasswordState> for GetTemporaryPasswordStateBuilder {
    fn as_ref(&self) -> &GetTemporaryPasswordState {
        &self.inner
    }
}
