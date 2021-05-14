use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns the current state of 2-step verification
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPasswordState {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetPasswordState {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetPasswordState {}

impl GetPasswordState {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetPasswordStateBuilder {
        let mut inner = GetPasswordState::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getPasswordState".to_string();

        RTDGetPasswordStateBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDGetPasswordStateBuilder {
    inner: GetPasswordState,
}

impl RTDGetPasswordStateBuilder {
    pub fn build(&self) -> GetPasswordState {
        self.inner.clone()
    }
}

impl AsRef<GetPasswordState> for GetPasswordState {
    fn as_ref(&self) -> &GetPasswordState {
        self
    }
}

impl AsRef<GetPasswordState> for RTDGetPasswordStateBuilder {
    fn as_ref(&self) -> &GetPasswordState {
        &self.inner
    }
}
