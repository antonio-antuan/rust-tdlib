use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sets the verbosity level of the internal logging of TDLib. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetLogVerbosityLevel {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// New value of the verbosity level for logging. Value 0 corresponds to fatal errors, value 1 corresponds to errors, value 2 corresponds to warnings and debug warnings, value 3 corresponds to informational, value 4 corresponds to debug, value 5 corresponds to verbose debug, value greater than 5 and up to 1023 can be used to enable even more logging

    #[serde(default)]
    new_verbosity_level: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetLogVerbosityLevel {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetLogVerbosityLevel {}

impl SetLogVerbosityLevel {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetLogVerbosityLevelBuilder {
        let mut inner = SetLogVerbosityLevel::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setLogVerbosityLevel".to_string();

        SetLogVerbosityLevelBuilder { inner }
    }

    pub fn new_verbosity_level(&self) -> i32 {
        self.new_verbosity_level
    }
}

#[doc(hidden)]
pub struct SetLogVerbosityLevelBuilder {
    inner: SetLogVerbosityLevel,
}

#[deprecated]
pub type RTDSetLogVerbosityLevelBuilder = SetLogVerbosityLevelBuilder;

impl SetLogVerbosityLevelBuilder {
    pub fn build(&self) -> SetLogVerbosityLevel {
        self.inner.clone()
    }

    pub fn new_verbosity_level(&mut self, new_verbosity_level: i32) -> &mut Self {
        self.inner.new_verbosity_level = new_verbosity_level;
        self
    }
}

impl AsRef<SetLogVerbosityLevel> for SetLogVerbosityLevel {
    fn as_ref(&self) -> &SetLogVerbosityLevel {
        self
    }
}

impl AsRef<SetLogVerbosityLevel> for SetLogVerbosityLevelBuilder {
    fn as_ref(&self) -> &SetLogVerbosityLevel {
        &self.inner
    }
}
