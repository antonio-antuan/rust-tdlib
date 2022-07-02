use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Starts screen sharing in a joined group call. Returns join response payload for tgcalls
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StartGroupCallScreenSharing {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group call identifier

    #[serde(default)]
    group_call_id: i32,
    /// Screen sharing audio channel synchronization source identifier; received from tgcalls

    #[serde(default)]
    audio_source_id: i32,
    /// Group call join payload; received from tgcalls

    #[serde(default)]
    payload: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for StartGroupCallScreenSharing {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for StartGroupCallScreenSharing {}

impl StartGroupCallScreenSharing {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StartGroupCallScreenSharingBuilder {
        let mut inner = StartGroupCallScreenSharing::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "startGroupCallScreenSharing".to_string();

        StartGroupCallScreenSharingBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }

    pub fn audio_source_id(&self) -> i32 {
        self.audio_source_id
    }

    pub fn payload(&self) -> &String {
        &self.payload
    }
}

#[doc(hidden)]
pub struct StartGroupCallScreenSharingBuilder {
    inner: StartGroupCallScreenSharing,
}

#[deprecated]
pub type RTDStartGroupCallScreenSharingBuilder = StartGroupCallScreenSharingBuilder;

impl StartGroupCallScreenSharingBuilder {
    pub fn build(&self) -> StartGroupCallScreenSharing {
        self.inner.clone()
    }

    pub fn group_call_id(&mut self, group_call_id: i32) -> &mut Self {
        self.inner.group_call_id = group_call_id;
        self
    }

    pub fn audio_source_id(&mut self, audio_source_id: i32) -> &mut Self {
        self.inner.audio_source_id = audio_source_id;
        self
    }

    pub fn payload<T: AsRef<str>>(&mut self, payload: T) -> &mut Self {
        self.inner.payload = payload.as_ref().to_string();
        self
    }
}

impl AsRef<StartGroupCallScreenSharing> for StartGroupCallScreenSharing {
    fn as_ref(&self) -> &StartGroupCallScreenSharing {
        self
    }
}

impl AsRef<StartGroupCallScreenSharing> for StartGroupCallScreenSharingBuilder {
    fn as_ref(&self) -> &StartGroupCallScreenSharing {
        &self.inner
    }
}
