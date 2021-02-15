use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains the result of a custom request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CustomRequestResult {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A JSON-serialized result
    result: String,
}

impl RObject for CustomRequestResult {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl CustomRequestResult {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCustomRequestResultBuilder {
        let mut inner = CustomRequestResult::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDCustomRequestResultBuilder { inner }
    }

    pub fn result(&self) -> &String {
        &self.result
    }
}

#[doc(hidden)]
pub struct RTDCustomRequestResultBuilder {
    inner: CustomRequestResult,
}

impl RTDCustomRequestResultBuilder {
    pub fn build(&self) -> CustomRequestResult {
        self.inner.clone()
    }

    pub fn result<T: AsRef<str>>(&mut self, result: T) -> &mut Self {
        self.inner.result = result.as_ref().to_string();
        self
    }
}

impl AsRef<CustomRequestResult> for CustomRequestResult {
    fn as_ref(&self) -> &CustomRequestResult {
        self
    }
}

impl AsRef<CustomRequestResult> for RTDCustomRequestResultBuilder {
    fn as_ref(&self) -> &CustomRequestResult {
        &self.inner
    }
}
