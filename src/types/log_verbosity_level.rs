use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a TDLib internal log verbosity level
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogVerbosityLevel {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Log verbosity level
    verbosity_level: i32,
}

impl RObject for LogVerbosityLevel {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl LogVerbosityLevel {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDLogVerbosityLevelBuilder {
        let mut inner = LogVerbosityLevel::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDLogVerbosityLevelBuilder { inner }
    }

    pub fn verbosity_level(&self) -> i32 {
        self.verbosity_level
    }
}

#[doc(hidden)]
pub struct RTDLogVerbosityLevelBuilder {
    inner: LogVerbosityLevel,
}

impl RTDLogVerbosityLevelBuilder {
    pub fn build(&self) -> LogVerbosityLevel {
        self.inner.clone()
    }

    pub fn verbosity_level(&mut self, verbosity_level: i32) -> &mut Self {
        self.inner.verbosity_level = verbosity_level;
        self
    }
}

impl AsRef<LogVerbosityLevel> for LogVerbosityLevel {
    fn as_ref(&self) -> &LogVerbosityLevel {
        self
    }
}

impl AsRef<LogVerbosityLevel> for RTDLogVerbosityLevelBuilder {
    fn as_ref(&self) -> &LogVerbosityLevel {
        &self.inner
    }
}
