use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns current verbosity level for a specified TDLib internal log tag. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLogTagVerbosityLevel {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Logging tag to change verbosity level
    tag: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetLogTagVerbosityLevel {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetLogTagVerbosityLevel {}

impl GetLogTagVerbosityLevel {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetLogTagVerbosityLevelBuilder {
        let mut inner = GetLogTagVerbosityLevel::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getLogTagVerbosityLevel".to_string();

        RTDGetLogTagVerbosityLevelBuilder { inner }
    }

    pub fn tag(&self) -> &String {
        &self.tag
    }
}

#[doc(hidden)]
pub struct RTDGetLogTagVerbosityLevelBuilder {
    inner: GetLogTagVerbosityLevel,
}

impl RTDGetLogTagVerbosityLevelBuilder {
    pub fn build(&self) -> GetLogTagVerbosityLevel {
        self.inner.clone()
    }

    pub fn tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
        self.inner.tag = tag.as_ref().to_string();
        self
    }
}

impl AsRef<GetLogTagVerbosityLevel> for GetLogTagVerbosityLevel {
    fn as_ref(&self) -> &GetLogTagVerbosityLevel {
        self
    }
}

impl AsRef<GetLogTagVerbosityLevel> for RTDGetLogTagVerbosityLevelBuilder {
    fn as_ref(&self) -> &GetLogTagVerbosityLevel {
        &self.inner
    }
}
