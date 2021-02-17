use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns the current authorization state; this is an offline request. For informational purposes only. Use updateAuthorizationState instead to maintain the current authorization state. Can be called before initialization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAuthorizationState {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetAuthorizationState {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDAuthorizationState for GetAuthorizationState {}

impl RFunction for GetAuthorizationState {}

impl GetAuthorizationState {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetAuthorizationStateBuilder {
        let mut inner = GetAuthorizationState::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getAuthorizationState".to_string();

        RTDGetAuthorizationStateBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDGetAuthorizationStateBuilder {
    inner: GetAuthorizationState,
}

impl RTDGetAuthorizationStateBuilder {
    pub fn build(&self) -> GetAuthorizationState {
        self.inner.clone()
    }
}

impl AsRef<GetAuthorizationState> for GetAuthorizationState {
    fn as_ref(&self) -> &GetAuthorizationState {
        self
    }
}

impl AsRef<GetAuthorizationState> for RTDGetAuthorizationStateBuilder {
    fn as_ref(&self) -> &GetAuthorizationState {
        &self.inner
    }
}
