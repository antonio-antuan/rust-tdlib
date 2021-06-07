use crate::errors::*;
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
    #[serde(rename(
        serialize = "callProblemDistortedSpeech",
        deserialize = "callProblemDistortedSpeech"
    ))]
    DistortedSpeech(CallProblemDistortedSpeech),
    /// The video was distorted
    #[serde(rename(
        serialize = "callProblemDistortedVideo",
        deserialize = "callProblemDistortedVideo"
    ))]
    DistortedVideo(CallProblemDistortedVideo),
    /// The call ended unexpectedly
    #[serde(rename(serialize = "callProblemDropped", deserialize = "callProblemDropped"))]
    Dropped(CallProblemDropped),
    /// The user heard their own voice
    #[serde(rename(serialize = "callProblemEcho", deserialize = "callProblemEcho"))]
    Echo(CallProblemEcho),
    /// The other side kept disappearing
    #[serde(rename(
        serialize = "callProblemInterruptions",
        deserialize = "callProblemInterruptions"
    ))]
    Interruptions(CallProblemInterruptions),
    /// The user heard background noise
    #[serde(rename(serialize = "callProblemNoise", deserialize = "callProblemNoise"))]
    Noise(CallProblemNoise),
    /// The video was pixelated
    #[serde(rename(
        serialize = "callProblemPixelatedVideo",
        deserialize = "callProblemPixelatedVideo"
    ))]
    PixelatedVideo(CallProblemPixelatedVideo),
    /// The user couldn't hear the other side
    #[serde(rename(
        serialize = "callProblemSilentLocal",
        deserialize = "callProblemSilentLocal"
    ))]
    SilentLocal(CallProblemSilentLocal),
    /// The other side couldn't hear the user
    #[serde(rename(
        serialize = "callProblemSilentRemote",
        deserialize = "callProblemSilentRemote"
    ))]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallProblemDistortedSpeechBuilder {
        let mut inner = CallProblemDistortedSpeech::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDCallProblemDistortedSpeechBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDCallProblemDistortedSpeechBuilder {
    inner: CallProblemDistortedSpeech,
}

impl RTDCallProblemDistortedSpeechBuilder {
    pub fn build(&self) -> CallProblemDistortedSpeech {
        self.inner.clone()
    }
}

impl AsRef<CallProblemDistortedSpeech> for CallProblemDistortedSpeech {
    fn as_ref(&self) -> &CallProblemDistortedSpeech {
        self
    }
}

impl AsRef<CallProblemDistortedSpeech> for RTDCallProblemDistortedSpeechBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallProblemDistortedVideoBuilder {
        let mut inner = CallProblemDistortedVideo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDCallProblemDistortedVideoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDCallProblemDistortedVideoBuilder {
    inner: CallProblemDistortedVideo,
}

impl RTDCallProblemDistortedVideoBuilder {
    pub fn build(&self) -> CallProblemDistortedVideo {
        self.inner.clone()
    }
}

impl AsRef<CallProblemDistortedVideo> for CallProblemDistortedVideo {
    fn as_ref(&self) -> &CallProblemDistortedVideo {
        self
    }
}

impl AsRef<CallProblemDistortedVideo> for RTDCallProblemDistortedVideoBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallProblemDroppedBuilder {
        let mut inner = CallProblemDropped::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDCallProblemDroppedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDCallProblemDroppedBuilder {
    inner: CallProblemDropped,
}

impl RTDCallProblemDroppedBuilder {
    pub fn build(&self) -> CallProblemDropped {
        self.inner.clone()
    }
}

impl AsRef<CallProblemDropped> for CallProblemDropped {
    fn as_ref(&self) -> &CallProblemDropped {
        self
    }
}

impl AsRef<CallProblemDropped> for RTDCallProblemDroppedBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallProblemEchoBuilder {
        let mut inner = CallProblemEcho::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDCallProblemEchoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDCallProblemEchoBuilder {
    inner: CallProblemEcho,
}

impl RTDCallProblemEchoBuilder {
    pub fn build(&self) -> CallProblemEcho {
        self.inner.clone()
    }
}

impl AsRef<CallProblemEcho> for CallProblemEcho {
    fn as_ref(&self) -> &CallProblemEcho {
        self
    }
}

impl AsRef<CallProblemEcho> for RTDCallProblemEchoBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallProblemInterruptionsBuilder {
        let mut inner = CallProblemInterruptions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDCallProblemInterruptionsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDCallProblemInterruptionsBuilder {
    inner: CallProblemInterruptions,
}

impl RTDCallProblemInterruptionsBuilder {
    pub fn build(&self) -> CallProblemInterruptions {
        self.inner.clone()
    }
}

impl AsRef<CallProblemInterruptions> for CallProblemInterruptions {
    fn as_ref(&self) -> &CallProblemInterruptions {
        self
    }
}

impl AsRef<CallProblemInterruptions> for RTDCallProblemInterruptionsBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallProblemNoiseBuilder {
        let mut inner = CallProblemNoise::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDCallProblemNoiseBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDCallProblemNoiseBuilder {
    inner: CallProblemNoise,
}

impl RTDCallProblemNoiseBuilder {
    pub fn build(&self) -> CallProblemNoise {
        self.inner.clone()
    }
}

impl AsRef<CallProblemNoise> for CallProblemNoise {
    fn as_ref(&self) -> &CallProblemNoise {
        self
    }
}

impl AsRef<CallProblemNoise> for RTDCallProblemNoiseBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallProblemPixelatedVideoBuilder {
        let mut inner = CallProblemPixelatedVideo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDCallProblemPixelatedVideoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDCallProblemPixelatedVideoBuilder {
    inner: CallProblemPixelatedVideo,
}

impl RTDCallProblemPixelatedVideoBuilder {
    pub fn build(&self) -> CallProblemPixelatedVideo {
        self.inner.clone()
    }
}

impl AsRef<CallProblemPixelatedVideo> for CallProblemPixelatedVideo {
    fn as_ref(&self) -> &CallProblemPixelatedVideo {
        self
    }
}

impl AsRef<CallProblemPixelatedVideo> for RTDCallProblemPixelatedVideoBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallProblemSilentLocalBuilder {
        let mut inner = CallProblemSilentLocal::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDCallProblemSilentLocalBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDCallProblemSilentLocalBuilder {
    inner: CallProblemSilentLocal,
}

impl RTDCallProblemSilentLocalBuilder {
    pub fn build(&self) -> CallProblemSilentLocal {
        self.inner.clone()
    }
}

impl AsRef<CallProblemSilentLocal> for CallProblemSilentLocal {
    fn as_ref(&self) -> &CallProblemSilentLocal {
        self
    }
}

impl AsRef<CallProblemSilentLocal> for RTDCallProblemSilentLocalBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallProblemSilentRemoteBuilder {
        let mut inner = CallProblemSilentRemote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDCallProblemSilentRemoteBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDCallProblemSilentRemoteBuilder {
    inner: CallProblemSilentRemote,
}

impl RTDCallProblemSilentRemoteBuilder {
    pub fn build(&self) -> CallProblemSilentRemote {
        self.inner.clone()
    }
}

impl AsRef<CallProblemSilentRemote> for CallProblemSilentRemote {
    fn as_ref(&self) -> &CallProblemSilentRemote {
        self
    }
}

impl AsRef<CallProblemSilentRemote> for RTDCallProblemSilentRemoteBuilder {
    fn as_ref(&self) -> &CallProblemSilentRemote {
        &self.inner
    }
}
