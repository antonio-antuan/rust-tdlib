use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Starts recording of an active group call. Requires groupCall.can_be_managed group call flag
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StartGroupCallRecording {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group call identifier

    #[serde(default)]
    group_call_id: i32,
    /// Group call recording title; 0-64 characters

    #[serde(default)]
    title: String,
    /// Pass true to record a video file instead of an audio file

    #[serde(default)]
    record_video: bool,
    /// Pass true to use portrait orientation for video instead of landscape one

    #[serde(default)]
    use_portrait_orientation: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for StartGroupCallRecording {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for StartGroupCallRecording {}

impl StartGroupCallRecording {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StartGroupCallRecordingBuilder {
        let mut inner = StartGroupCallRecording::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "startGroupCallRecording".to_string();

        StartGroupCallRecordingBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn record_video(&self) -> bool {
        self.record_video
    }

    pub fn use_portrait_orientation(&self) -> bool {
        self.use_portrait_orientation
    }
}

#[doc(hidden)]
pub struct StartGroupCallRecordingBuilder {
    inner: StartGroupCallRecording,
}

#[deprecated]
pub type RTDStartGroupCallRecordingBuilder = StartGroupCallRecordingBuilder;

impl StartGroupCallRecordingBuilder {
    pub fn build(&self) -> StartGroupCallRecording {
        self.inner.clone()
    }

    pub fn group_call_id(&mut self, group_call_id: i32) -> &mut Self {
        self.inner.group_call_id = group_call_id;
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn record_video(&mut self, record_video: bool) -> &mut Self {
        self.inner.record_video = record_video;
        self
    }

    pub fn use_portrait_orientation(&mut self, use_portrait_orientation: bool) -> &mut Self {
        self.inner.use_portrait_orientation = use_portrait_orientation;
        self
    }
}

impl AsRef<StartGroupCallRecording> for StartGroupCallRecording {
    fn as_ref(&self) -> &StartGroupCallRecording {
        self
    }
}

impl AsRef<StartGroupCallRecording> for StartGroupCallRecordingBuilder {
    fn as_ref(&self) -> &StartGroupCallRecording {
        &self.inner
    }
}
