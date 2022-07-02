use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns all updates needed to restore current TDLib state, i.e. all actual UpdateAuthorizationState/UpdateUser/UpdateNewChat and others. This is especially useful if TDLib is run in a separate process. Can be called before initialization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetCurrentState {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetCurrentState {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetCurrentState {}

impl GetCurrentState {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetCurrentStateBuilder {
        let mut inner = GetCurrentState::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getCurrentState".to_string();

        GetCurrentStateBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetCurrentStateBuilder {
    inner: GetCurrentState,
}

#[deprecated]
pub type RTDGetCurrentStateBuilder = GetCurrentStateBuilder;

impl GetCurrentStateBuilder {
    pub fn build(&self) -> GetCurrentState {
        self.inner.clone()
    }
}

impl AsRef<GetCurrentState> for GetCurrentState {
    fn as_ref(&self) -> &GetCurrentState {
        self
    }
}

impl AsRef<GetCurrentState> for GetCurrentStateBuilder {
    fn as_ref(&self) -> &GetCurrentState {
        &self.inner
    }
}
