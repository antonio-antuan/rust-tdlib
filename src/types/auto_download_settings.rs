use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains auto-download settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutoDownloadSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if the auto-download is enabled

    #[serde(default)]
    is_auto_download_enabled: bool,
    /// The maximum size of a photo file to be auto-downloaded, in bytes

    #[serde(default)]
    max_photo_file_size: i32,
    /// The maximum size of a video file to be auto-downloaded, in bytes

    #[serde(default)]
    max_video_file_size: i32,
    /// The maximum size of other file types to be auto-downloaded, in bytes

    #[serde(default)]
    max_other_file_size: i32,
    /// The maximum suggested bitrate for uploaded videos, in kbit/s

    #[serde(default)]
    video_upload_bitrate: i32,
    /// True, if the beginning of video files needs to be preloaded for instant playback

    #[serde(default)]
    preload_large_videos: bool,
    /// True, if the next audio track needs to be preloaded while the user is listening to an audio file

    #[serde(default)]
    preload_next_audio: bool,
    /// True, if "use less data for calls" option needs to be enabled

    #[serde(default)]
    use_less_data_for_calls: bool,
}

impl RObject for AutoDownloadSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl AutoDownloadSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AutoDownloadSettingsBuilder {
        let mut inner = AutoDownloadSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AutoDownloadSettingsBuilder { inner }
    }

    pub fn is_auto_download_enabled(&self) -> bool {
        self.is_auto_download_enabled
    }

    pub fn max_photo_file_size(&self) -> i32 {
        self.max_photo_file_size
    }

    pub fn max_video_file_size(&self) -> i32 {
        self.max_video_file_size
    }

    pub fn max_other_file_size(&self) -> i32 {
        self.max_other_file_size
    }

    pub fn video_upload_bitrate(&self) -> i32 {
        self.video_upload_bitrate
    }

    pub fn preload_large_videos(&self) -> bool {
        self.preload_large_videos
    }

    pub fn preload_next_audio(&self) -> bool {
        self.preload_next_audio
    }

    pub fn use_less_data_for_calls(&self) -> bool {
        self.use_less_data_for_calls
    }
}

#[doc(hidden)]
pub struct AutoDownloadSettingsBuilder {
    inner: AutoDownloadSettings,
}

#[deprecated]
pub type RTDAutoDownloadSettingsBuilder = AutoDownloadSettingsBuilder;

impl AutoDownloadSettingsBuilder {
    pub fn build(&self) -> AutoDownloadSettings {
        self.inner.clone()
    }

    pub fn is_auto_download_enabled(&mut self, is_auto_download_enabled: bool) -> &mut Self {
        self.inner.is_auto_download_enabled = is_auto_download_enabled;
        self
    }

    pub fn max_photo_file_size(&mut self, max_photo_file_size: i32) -> &mut Self {
        self.inner.max_photo_file_size = max_photo_file_size;
        self
    }

    pub fn max_video_file_size(&mut self, max_video_file_size: i32) -> &mut Self {
        self.inner.max_video_file_size = max_video_file_size;
        self
    }

    pub fn max_other_file_size(&mut self, max_other_file_size: i32) -> &mut Self {
        self.inner.max_other_file_size = max_other_file_size;
        self
    }

    pub fn video_upload_bitrate(&mut self, video_upload_bitrate: i32) -> &mut Self {
        self.inner.video_upload_bitrate = video_upload_bitrate;
        self
    }

    pub fn preload_large_videos(&mut self, preload_large_videos: bool) -> &mut Self {
        self.inner.preload_large_videos = preload_large_videos;
        self
    }

    pub fn preload_next_audio(&mut self, preload_next_audio: bool) -> &mut Self {
        self.inner.preload_next_audio = preload_next_audio;
        self
    }

    pub fn use_less_data_for_calls(&mut self, use_less_data_for_calls: bool) -> &mut Self {
        self.inner.use_less_data_for_calls = use_less_data_for_calls;
        self
    }
}

impl AsRef<AutoDownloadSettings> for AutoDownloadSettings {
    fn as_ref(&self) -> &AutoDownloadSettings {
        self
    }
}

impl AsRef<AutoDownloadSettings> for AutoDownloadSettingsBuilder {
    fn as_ref(&self) -> &AutoDownloadSettings {
        &self.inner
    }
}
