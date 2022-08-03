use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes the type of a call server
pub trait TDCallServerType: Debug + RObject {}

/// Describes the type of a call server
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CallServerType {
    #[doc(hidden)]
    _Default,
    /// A Telegram call reflector
    #[serde(rename = "callServerTypeTelegramReflector")]
    TelegramReflector(CallServerTypeTelegramReflector),
    /// A WebRTC server
    #[serde(rename = "callServerTypeWebrtc")]
    Webrtc(CallServerTypeWebrtc),
}

impl Default for CallServerType {
    fn default() -> Self {
        CallServerType::_Default
    }
}

impl RObject for CallServerType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            CallServerType::TelegramReflector(t) => t.extra(),
            CallServerType::Webrtc(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            CallServerType::TelegramReflector(t) => t.client_id(),
            CallServerType::Webrtc(t) => t.client_id(),

            _ => None,
        }
    }
}

impl CallServerType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, CallServerType::_Default)
    }
}

impl AsRef<CallServerType> for CallServerType {
    fn as_ref(&self) -> &CallServerType {
        self
    }
}

/// A Telegram call reflector
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallServerTypeTelegramReflector {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A peer tag to be used with the reflector

    #[serde(default)]
    peer_tag: String,
}

impl RObject for CallServerTypeTelegramReflector {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCallServerType for CallServerTypeTelegramReflector {}

impl CallServerTypeTelegramReflector {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CallServerTypeTelegramReflectorBuilder {
        let mut inner = CallServerTypeTelegramReflector::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CallServerTypeTelegramReflectorBuilder { inner }
    }

    pub fn peer_tag(&self) -> &String {
        &self.peer_tag
    }
}

#[doc(hidden)]
pub struct CallServerTypeTelegramReflectorBuilder {
    inner: CallServerTypeTelegramReflector,
}

#[deprecated]
pub type RTDCallServerTypeTelegramReflectorBuilder = CallServerTypeTelegramReflectorBuilder;

impl CallServerTypeTelegramReflectorBuilder {
    pub fn build(&self) -> CallServerTypeTelegramReflector {
        self.inner.clone()
    }

    pub fn peer_tag<T: AsRef<str>>(&mut self, peer_tag: T) -> &mut Self {
        self.inner.peer_tag = peer_tag.as_ref().to_string();
        self
    }
}

impl AsRef<CallServerTypeTelegramReflector> for CallServerTypeTelegramReflector {
    fn as_ref(&self) -> &CallServerTypeTelegramReflector {
        self
    }
}

impl AsRef<CallServerTypeTelegramReflector> for CallServerTypeTelegramReflectorBuilder {
    fn as_ref(&self) -> &CallServerTypeTelegramReflector {
        &self.inner
    }
}

/// A WebRTC server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallServerTypeWebrtc {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Username to be used for authentication

    #[serde(default)]
    username: String,
    /// Authentication password

    #[serde(default)]
    password: String,
    /// True, if the server supports TURN

    #[serde(default)]
    supports_turn: bool,
    /// True, if the server supports STUN

    #[serde(default)]
    supports_stun: bool,
}

impl RObject for CallServerTypeWebrtc {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCallServerType for CallServerTypeWebrtc {}

impl CallServerTypeWebrtc {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CallServerTypeWebrtcBuilder {
        let mut inner = CallServerTypeWebrtc::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CallServerTypeWebrtcBuilder { inner }
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn password(&self) -> &String {
        &self.password
    }

    pub fn supports_turn(&self) -> bool {
        self.supports_turn
    }

    pub fn supports_stun(&self) -> bool {
        self.supports_stun
    }
}

#[doc(hidden)]
pub struct CallServerTypeWebrtcBuilder {
    inner: CallServerTypeWebrtc,
}

#[deprecated]
pub type RTDCallServerTypeWebrtcBuilder = CallServerTypeWebrtcBuilder;

impl CallServerTypeWebrtcBuilder {
    pub fn build(&self) -> CallServerTypeWebrtc {
        self.inner.clone()
    }

    pub fn username<T: AsRef<str>>(&mut self, username: T) -> &mut Self {
        self.inner.username = username.as_ref().to_string();
        self
    }

    pub fn password<T: AsRef<str>>(&mut self, password: T) -> &mut Self {
        self.inner.password = password.as_ref().to_string();
        self
    }

    pub fn supports_turn(&mut self, supports_turn: bool) -> &mut Self {
        self.inner.supports_turn = supports_turn;
        self
    }

    pub fn supports_stun(&mut self, supports_stun: bool) -> &mut Self {
        self.inner.supports_stun = supports_stun;
        self
    }
}

impl AsRef<CallServerTypeWebrtc> for CallServerTypeWebrtc {
    fn as_ref(&self) -> &CallServerTypeWebrtc {
        self
    }
}

impl AsRef<CallServerTypeWebrtc> for CallServerTypeWebrtcBuilder {
    fn as_ref(&self) -> &CallServerTypeWebrtc {
        &self.inner
    }
}
