use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of available TDLib internal log tags
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogTags {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of log tags

    #[serde(default)]
    tags: Vec<String>,
}

impl RObject for LogTags {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl LogTags {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> LogTagsBuilder {
        let mut inner = LogTags::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        LogTagsBuilder { inner }
    }

    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }
}

#[doc(hidden)]
pub struct LogTagsBuilder {
    inner: LogTags,
}

#[deprecated]
pub type RTDLogTagsBuilder = LogTagsBuilder;

impl LogTagsBuilder {
    pub fn build(&self) -> LogTags {
        self.inner.clone()
    }

    pub fn tags(&mut self, tags: Vec<String>) -> &mut Self {
        self.inner.tags = tags;
        self
    }
}

impl AsRef<LogTags> for LogTags {
    fn as_ref(&self) -> &LogTags {
        self
    }
}

impl AsRef<LogTags> for LogTagsBuilder {
    fn as_ref(&self) -> &LogTags {
        &self.inner
    }
}
