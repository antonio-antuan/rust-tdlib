use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sets the verbosity level for a specified TDLib internal log tag. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetLogTagVerbosityLevel {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Logging tag to change verbosity level

    #[serde(default)]
    tag: String,
    /// New verbosity level; 1-1024

    #[serde(default)]
    new_verbosity_level: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetLogTagVerbosityLevel {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetLogTagVerbosityLevel {}

impl SetLogTagVerbosityLevel {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetLogTagVerbosityLevelBuilder {
        let mut inner = SetLogTagVerbosityLevel::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setLogTagVerbosityLevel".to_string();

        SetLogTagVerbosityLevelBuilder { inner }
    }

    pub fn tag(&self) -> &String {
        &self.tag
    }

    pub fn new_verbosity_level(&self) -> i32 {
        self.new_verbosity_level
    }
}

#[doc(hidden)]
pub struct SetLogTagVerbosityLevelBuilder {
    inner: SetLogTagVerbosityLevel,
}

#[deprecated]
pub type RTDSetLogTagVerbosityLevelBuilder = SetLogTagVerbosityLevelBuilder;

impl SetLogTagVerbosityLevelBuilder {
    pub fn build(&self) -> SetLogTagVerbosityLevel {
        self.inner.clone()
    }

    pub fn tag<T: AsRef<str>>(&mut self, tag: T) -> &mut Self {
        self.inner.tag = tag.as_ref().to_string();
        self
    }

    pub fn new_verbosity_level(&mut self, new_verbosity_level: i32) -> &mut Self {
        self.inner.new_verbosity_level = new_verbosity_level;
        self
    }
}

impl AsRef<SetLogTagVerbosityLevel> for SetLogTagVerbosityLevel {
    fn as_ref(&self) -> &SetLogTagVerbosityLevel {
        self
    }
}

impl AsRef<SetLogTagVerbosityLevel> for SetLogTagVerbosityLevelBuilder {
    fn as_ref(&self) -> &SetLogTagVerbosityLevel {
        &self.inner
    }
}
