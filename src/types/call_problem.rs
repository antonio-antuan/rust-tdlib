use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes the exact type of a problem with a call
pub trait TDCallProblem: Debug + RObject {}

/// Describes the exact type of a problem with a call
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CallProblem {
    #[doc(hidden)]
    _Default,
    /// The speech was distorted
    #[serde(rename = "callProblemDistortedSpeech")]
    DistortedSpeech(CallProblemDistortedSpeech),
    /// The video was distorted
    #[serde(rename = "callProblemDistortedVideo")]
    DistortedVideo(CallProblemDistortedVideo),
    /// The call ended unexpectedly
    #[serde(rename = "callProblemDropped")]
    Dropped(CallProblemDropped),
    /// The user heard their own voice
    #[serde(rename = "callProblemEcho")]
    Echo(CallProblemEcho),
    /// The other side kept disappearing
    #[serde(rename = "callProblemInterruptions")]
    Interruptions(CallProblemInterruptions),
    /// The user heard background noise
    #[serde(rename = "callProblemNoise")]
    Noise(CallProblemNoise),
    /// The video was pixelated
    #[serde(rename = "callProblemPixelatedVideo")]
    PixelatedVideo(CallProblemPixelatedVideo),
    /// The user couldn't hear the other side
    #[serde(rename = "callProblemSilentLocal")]
    SilentLocal(CallProblemSilentLocal),
    /// The other side couldn't hear the user
    #[serde(rename = "callProblemSilentRemote")]
    SilentRemote(CallProblemSilentRemote),
}

impl Default for CallProblem {
    fn default() -> Self {
        CallProblem::_Default
    }
}

impl RObject for CallProblem {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            CallProblem::DistortedSpeech(t) => t.extra(),
            CallProblem::DistortedVideo(t) => t.extra(),
            CallProblem::Dropped(t) => t.extra(),
            CallProblem::Echo(t) => t.extra(),
            CallProblem::Interruptions(t) => t.extra(),
            CallProblem::Noise(t) => t.extra(),
            CallProblem::PixelatedVideo(t) => t.extra(),
            CallProblem::SilentLocal(t) => t.extra(),
            CallProblem::SilentRemote(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            CallProblem::DistortedSpeech(t) => t.client_id(),
            CallProblem::DistortedVideo(t) => t.client_id(),
            CallProblem::Dropped(t) => t.client_id(),
            CallProblem::Echo(t) => t.client_id(),
            CallProblem::Interruptions(t) => t.client_id(),
            CallProblem::Noise(t) => t.client_id(),
            CallProblem::PixelatedVideo(t) => t.client_id(),
            CallProblem::SilentLocal(t) => t.client_id(),
            CallProblem::SilentRemote(t) => t.client_id(),

            _ => None,
        }
    }
}

impl CallProblem {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, CallProblem::_Default)
    }
}

impl AsRef<CallProblem> for CallProblem {
    fn as_ref(&self) -> &CallProblem {
        self
    }
}

/// The speech was distorted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallProblemDistortedSpeech {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CallProblemDistortedSpeech {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCallProblem for CallProblemDistortedSpeech {}

impl CallProblemDistortedSpeech {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CallProblemDistortedSpeechBuilder {
        let mut inner = CallProblemDistortedSpeech::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CallProblemDistortedSpeechBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CallProblemDistortedSpeechBuilder {
    inner: CallProblemDistortedSpeech,
}

#[deprecated]
pub type RTDCallProblemDistortedSpeechBuilder = CallProblemDistortedSpeechBuilder;

impl CallProblemDistortedSpeechBuilder {
    pub fn build(&self) -> CallProblemDistortedSpeech {
        self.inner.clone()
    }
}

impl AsRef<CallProblemDistortedSpeech> for CallProblemDistortedSpeech {
    fn as_ref(&self) -> &CallProblemDistortedSpeech {
        self
    }
}

impl AsRef<CallProblemDistortedSpeech> for CallProblemDistortedSpeechBuilder {
    fn as_ref(&self) -> &CallProblemDistortedSpeech {
        &self.inner
    }
}

/// The video was distorted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallProblemDistortedVideo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CallProblemDistortedVideo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCallProblem for CallProblemDistortedVideo {}

impl CallProblemDistortedVideo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CallProblemDistortedVideoBuilder {
        let mut inner = CallProblemDistortedVideo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CallProblemDistortedVideoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CallProblemDistortedVideoBuilder {
    inner: CallProblemDistortedVideo,
}

#[deprecated]
pub type RTDCallProblemDistortedVideoBuilder = CallProblemDistortedVideoBuilder;

impl CallProblemDistortedVideoBuilder {
    pub fn build(&self) -> CallProblemDistortedVideo {
        self.inner.clone()
    }
}

impl AsRef<CallProblemDistortedVideo> for CallProblemDistortedVideo {
    fn as_ref(&self) -> &CallProblemDistortedVideo {
        self
    }
}

impl AsRef<CallProblemDistortedVideo> for CallProblemDistortedVideoBuilder {
    fn as_ref(&self) -> &CallProblemDistortedVideo {
        &self.inner
    }
}

/// The call ended unexpectedly
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallProblemDropped {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CallProblemDropped {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCallProblem for CallProblemDropped {}

impl CallProblemDropped {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CallProblemDroppedBuilder {
        let mut inner = CallProblemDropped::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CallProblemDroppedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CallProblemDroppedBuilder {
    inner: CallProblemDropped,
}

#[deprecated]
pub type RTDCallProblemDroppedBuilder = CallProblemDroppedBuilder;

impl CallProblemDroppedBuilder {
    pub fn build(&self) -> CallProblemDropped {
        self.inner.clone()
    }
}

impl AsRef<CallProblemDropped> for CallProblemDropped {
    fn as_ref(&self) -> &CallProblemDropped {
        self
    }
}

impl AsRef<CallProblemDropped> for CallProblemDroppedBuilder {
    fn as_ref(&self) -> &CallProblemDropped {
        &self.inner
    }
}

/// The user heard their own voice
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallProblemEcho {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CallProblemEcho {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCallProblem for CallProblemEcho {}

impl CallProblemEcho {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CallProblemEchoBuilder {
        let mut inner = CallProblemEcho::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CallProblemEchoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CallProblemEchoBuilder {
    inner: CallProblemEcho,
}

#[deprecated]
pub type RTDCallProblemEchoBuilder = CallProblemEchoBuilder;

impl CallProblemEchoBuilder {
    pub fn build(&self) -> CallProblemEcho {
        self.inner.clone()
    }
}

impl AsRef<CallProblemEcho> for CallProblemEcho {
    fn as_ref(&self) -> &CallProblemEcho {
        self
    }
}

impl AsRef<CallProblemEcho> for CallProblemEchoBuilder {
    fn as_ref(&self) -> &CallProblemEcho {
        &self.inner
    }
}

/// The other side kept disappearing
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallProblemInterruptions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CallProblemInterruptions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCallProblem for CallProblemInterruptions {}

impl CallProblemInterruptions {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CallProblemInterruptionsBuilder {
        let mut inner = CallProblemInterruptions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CallProblemInterruptionsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CallProblemInterruptionsBuilder {
    inner: CallProblemInterruptions,
}

#[deprecated]
pub type RTDCallProblemInterruptionsBuilder = CallProblemInterruptionsBuilder;

impl CallProblemInterruptionsBuilder {
    pub fn build(&self) -> CallProblemInterruptions {
        self.inner.clone()
    }
}

impl AsRef<CallProblemInterruptions> for CallProblemInterruptions {
    fn as_ref(&self) -> &CallProblemInterruptions {
        self
    }
}

impl AsRef<CallProblemInterruptions> for CallProblemInterruptionsBuilder {
    fn as_ref(&self) -> &CallProblemInterruptions {
        &self.inner
    }
}

/// The user heard background noise
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallProblemNoise {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CallProblemNoise {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCallProblem for CallProblemNoise {}

impl CallProblemNoise {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CallProblemNoiseBuilder {
        let mut inner = CallProblemNoise::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CallProblemNoiseBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CallProblemNoiseBuilder {
    inner: CallProblemNoise,
}

#[deprecated]
pub type RTDCallProblemNoiseBuilder = CallProblemNoiseBuilder;

impl CallProblemNoiseBuilder {
    pub fn build(&self) -> CallProblemNoise {
        self.inner.clone()
    }
}

impl AsRef<CallProblemNoise> for CallProblemNoise {
    fn as_ref(&self) -> &CallProblemNoise {
        self
    }
}

impl AsRef<CallProblemNoise> for CallProblemNoiseBuilder {
    fn as_ref(&self) -> &CallProblemNoise {
        &self.inner
    }
}

/// The video was pixelated
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallProblemPixelatedVideo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CallProblemPixelatedVideo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCallProblem for CallProblemPixelatedVideo {}

impl CallProblemPixelatedVideo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CallProblemPixelatedVideoBuilder {
        let mut inner = CallProblemPixelatedVideo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CallProblemPixelatedVideoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CallProblemPixelatedVideoBuilder {
    inner: CallProblemPixelatedVideo,
}

#[deprecated]
pub type RTDCallProblemPixelatedVideoBuilder = CallProblemPixelatedVideoBuilder;

impl CallProblemPixelatedVideoBuilder {
    pub fn build(&self) -> CallProblemPixelatedVideo {
        self.inner.clone()
    }
}

impl AsRef<CallProblemPixelatedVideo> for CallProblemPixelatedVideo {
    fn as_ref(&self) -> &CallProblemPixelatedVideo {
        self
    }
}

impl AsRef<CallProblemPixelatedVideo> for CallProblemPixelatedVideoBuilder {
    fn as_ref(&self) -> &CallProblemPixelatedVideo {
        &self.inner
    }
}

/// The user couldn't hear the other side
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallProblemSilentLocal {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CallProblemSilentLocal {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCallProblem for CallProblemSilentLocal {}

impl CallProblemSilentLocal {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CallProblemSilentLocalBuilder {
        let mut inner = CallProblemSilentLocal::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CallProblemSilentLocalBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CallProblemSilentLocalBuilder {
    inner: CallProblemSilentLocal,
}

#[deprecated]
pub type RTDCallProblemSilentLocalBuilder = CallProblemSilentLocalBuilder;

impl CallProblemSilentLocalBuilder {
    pub fn build(&self) -> CallProblemSilentLocal {
        self.inner.clone()
    }
}

impl AsRef<CallProblemSilentLocal> for CallProblemSilentLocal {
    fn as_ref(&self) -> &CallProblemSilentLocal {
        self
    }
}

impl AsRef<CallProblemSilentLocal> for CallProblemSilentLocalBuilder {
    fn as_ref(&self) -> &CallProblemSilentLocal {
        &self.inner
    }
}

/// The other side couldn't hear the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallProblemSilentRemote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CallProblemSilentRemote {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCallProblem for CallProblemSilentRemote {}

impl CallProblemSilentRemote {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CallProblemSilentRemoteBuilder {
        let mut inner = CallProblemSilentRemote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CallProblemSilentRemoteBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CallProblemSilentRemoteBuilder {
    inner: CallProblemSilentRemote,
}

#[deprecated]
pub type RTDCallProblemSilentRemoteBuilder = CallProblemSilentRemoteBuilder;

impl CallProblemSilentRemoteBuilder {
    pub fn build(&self) -> CallProblemSilentRemote {
        self.inner.clone()
    }
}

impl AsRef<CallProblemSilentRemote> for CallProblemSilentRemote {
    fn as_ref(&self) -> &CallProblemSilentRemote {
        self
    }
}

impl AsRef<CallProblemSilentRemote> for CallProblemSilentRemoteBuilder {
    fn as_ref(&self) -> &CallProblemSilentRemote {
        &self.inner
    }
}
