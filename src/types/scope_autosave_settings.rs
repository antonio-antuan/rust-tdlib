use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains autosave settings for an autosave settings scope
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScopeAutosaveSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if photo autosave is enabled

    #[serde(default)]
    autosave_photos: bool,
    /// True, if video autosave is enabled

    #[serde(default)]
    autosave_videos: bool,
    /// The maximum size of a video file to be autosaved, in bytes; 512 KB  4000 MB

    #[serde(default)]
    max_video_file_size: i64,
}

impl RObject for ScopeAutosaveSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ScopeAutosaveSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ScopeAutosaveSettingsBuilder {
        let mut inner = ScopeAutosaveSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ScopeAutosaveSettingsBuilder { inner }
    }

    pub fn autosave_photos(&self) -> bool {
        self.autosave_photos
    }

    pub fn autosave_videos(&self) -> bool {
        self.autosave_videos
    }

    pub fn max_video_file_size(&self) -> i64 {
        self.max_video_file_size
    }
}

#[doc(hidden)]
pub struct ScopeAutosaveSettingsBuilder {
    inner: ScopeAutosaveSettings,
}

#[deprecated]
pub type RTDScopeAutosaveSettingsBuilder = ScopeAutosaveSettingsBuilder;

impl ScopeAutosaveSettingsBuilder {
    pub fn build(&self) -> ScopeAutosaveSettings {
        self.inner.clone()
    }

    pub fn autosave_photos(&mut self, autosave_photos: bool) -> &mut Self {
        self.inner.autosave_photos = autosave_photos;
        self
    }

    pub fn autosave_videos(&mut self, autosave_videos: bool) -> &mut Self {
        self.inner.autosave_videos = autosave_videos;
        self
    }

    pub fn max_video_file_size(&mut self, max_video_file_size: i64) -> &mut Self {
        self.inner.max_video_file_size = max_video_file_size;
        self
    }
}

impl AsRef<ScopeAutosaveSettings> for ScopeAutosaveSettings {
    fn as_ref(&self) -> &ScopeAutosaveSettings {
        self
    }
}

impl AsRef<ScopeAutosaveSettings> for ScopeAutosaveSettingsBuilder {
    fn as_ref(&self) -> &ScopeAutosaveSettings {
        &self.inner
    }
}
