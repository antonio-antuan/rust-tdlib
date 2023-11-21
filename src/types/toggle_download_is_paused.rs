use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes pause state of a file in the file download list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleDownloadIsPaused {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the downloaded file

    #[serde(default)]
    file_id: i32,
    /// Pass true if the download is paused

    #[serde(default)]
    is_paused: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleDownloadIsPaused {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleDownloadIsPaused {}

impl ToggleDownloadIsPaused {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ToggleDownloadIsPausedBuilder {
        let mut inner = ToggleDownloadIsPaused::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleDownloadIsPaused".to_string();

        ToggleDownloadIsPausedBuilder { inner }
    }

    pub fn file_id(&self) -> i32 {
        self.file_id
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }
}

#[doc(hidden)]
pub struct ToggleDownloadIsPausedBuilder {
    inner: ToggleDownloadIsPaused,
}

#[deprecated]
pub type RTDToggleDownloadIsPausedBuilder = ToggleDownloadIsPausedBuilder;

impl ToggleDownloadIsPausedBuilder {
    pub fn build(&self) -> ToggleDownloadIsPaused {
        self.inner.clone()
    }

    pub fn file_id(&mut self, file_id: i32) -> &mut Self {
        self.inner.file_id = file_id;
        self
    }

    pub fn is_paused(&mut self, is_paused: bool) -> &mut Self {
        self.inner.is_paused = is_paused;
        self
    }
}

impl AsRef<ToggleDownloadIsPaused> for ToggleDownloadIsPaused {
    fn as_ref(&self) -> &ToggleDownloadIsPaused {
        self
    }
}

impl AsRef<ToggleDownloadIsPaused> for ToggleDownloadIsPausedBuilder {
    fn as_ref(&self) -> &ToggleDownloadIsPaused {
        &self.inner
    }
}
