use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes pause state of all files in the file download list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleAllDownloadsArePaused {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Pass true to pause all downloads; pass false to unpause them
    are_paused: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleAllDownloadsArePaused {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleAllDownloadsArePaused {}

impl ToggleAllDownloadsArePaused {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDToggleAllDownloadsArePausedBuilder {
        let mut inner = ToggleAllDownloadsArePaused::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleAllDownloadsArePaused".to_string();

        RTDToggleAllDownloadsArePausedBuilder { inner }
    }

    pub fn are_paused(&self) -> bool {
        self.are_paused
    }
}

#[doc(hidden)]
pub struct RTDToggleAllDownloadsArePausedBuilder {
    inner: ToggleAllDownloadsArePaused,
}

impl RTDToggleAllDownloadsArePausedBuilder {
    pub fn build(&self) -> ToggleAllDownloadsArePaused {
        self.inner.clone()
    }

    pub fn are_paused(&mut self, are_paused: bool) -> &mut Self {
        self.inner.are_paused = are_paused;
        self
    }
}

impl AsRef<ToggleAllDownloadsArePaused> for ToggleAllDownloadsArePaused {
    fn as_ref(&self) -> &ToggleAllDownloadsArePaused {
        self
    }
}

impl AsRef<ToggleAllDownloadsArePaused> for RTDToggleAllDownloadsArePausedBuilder {
    fn as_ref(&self) -> &ToggleAllDownloadsArePaused {
        &self.inner
    }
}
