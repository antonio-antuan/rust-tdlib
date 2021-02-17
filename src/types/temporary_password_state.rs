use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns information about the availability of a temporary password, which can be used for payments
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TemporaryPasswordState {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if a temporary password is available
    has_password: bool,
    /// Time left before the temporary password expires, in seconds
    valid_for: i32,
}

impl RObject for TemporaryPasswordState {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TemporaryPasswordState {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTemporaryPasswordStateBuilder {
        let mut inner = TemporaryPasswordState::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDTemporaryPasswordStateBuilder { inner }
    }

    pub fn has_password(&self) -> bool {
        self.has_password
    }

    pub fn valid_for(&self) -> i32 {
        self.valid_for
    }
}

#[doc(hidden)]
pub struct RTDTemporaryPasswordStateBuilder {
    inner: TemporaryPasswordState,
}

impl RTDTemporaryPasswordStateBuilder {
    pub fn build(&self) -> TemporaryPasswordState {
        self.inner.clone()
    }

    pub fn has_password(&mut self, has_password: bool) -> &mut Self {
        self.inner.has_password = has_password;
        self
    }

    pub fn valid_for(&mut self, valid_for: i32) -> &mut Self {
        self.inner.valid_for = valid_for;
        self
    }
}

impl AsRef<TemporaryPasswordState> for TemporaryPasswordState {
    fn as_ref(&self) -> &TemporaryPasswordState {
        self
    }
}

impl AsRef<TemporaryPasswordState> for RTDTemporaryPasswordStateBuilder {
    fn as_ref(&self) -> &TemporaryPasswordState {
        &self.inner
    }
}
