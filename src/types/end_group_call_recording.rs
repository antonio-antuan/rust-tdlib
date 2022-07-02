use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Ends recording of an active group call. Requires groupCall.can_be_managed group call flag
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EndGroupCallRecording {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group call identifier

    #[serde(default)]
    group_call_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for EndGroupCallRecording {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for EndGroupCallRecording {}

impl EndGroupCallRecording {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EndGroupCallRecordingBuilder {
        let mut inner = EndGroupCallRecording::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "endGroupCallRecording".to_string();

        EndGroupCallRecordingBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }
}

#[doc(hidden)]
pub struct EndGroupCallRecordingBuilder {
    inner: EndGroupCallRecording,
}

#[deprecated]
pub type RTDEndGroupCallRecordingBuilder = EndGroupCallRecordingBuilder;

impl EndGroupCallRecordingBuilder {
    pub fn build(&self) -> EndGroupCallRecording {
        self.inner.clone()
    }

    pub fn group_call_id(&mut self, group_call_id: i32) -> &mut Self {
        self.inner.group_call_id = group_call_id;
        self
    }
}

impl AsRef<EndGroupCallRecording> for EndGroupCallRecording {
    fn as_ref(&self) -> &EndGroupCallRecording {
        self
    }
}

impl AsRef<EndGroupCallRecording> for EndGroupCallRecordingBuilder {
    fn as_ref(&self) -> &EndGroupCallRecording {
        &self.inner
    }
}
