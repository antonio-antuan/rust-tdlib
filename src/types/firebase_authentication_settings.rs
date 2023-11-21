use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains settings for Firebase Authentication in the official applications
pub trait TDFirebaseAuthenticationSettings: Debug + RObject {}

/// Contains settings for Firebase Authentication in the official applications
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum FirebaseAuthenticationSettings {
    #[doc(hidden)]
    #[default]
    _Default,
    /// Settings for Firebase Authentication in the official Android application
    #[serde(rename = "firebaseAuthenticationSettingsAndroid")]
    Android(FirebaseAuthenticationSettingsAndroid),
    /// Settings for Firebase Authentication in the official iOS application
    #[serde(rename = "firebaseAuthenticationSettingsIos")]
    Ios(FirebaseAuthenticationSettingsIos),
}

impl RObject for FirebaseAuthenticationSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            FirebaseAuthenticationSettings::Android(t) => t.extra(),
            FirebaseAuthenticationSettings::Ios(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            FirebaseAuthenticationSettings::Android(t) => t.client_id(),
            FirebaseAuthenticationSettings::Ios(t) => t.client_id(),

            _ => None,
        }
    }
}

impl FirebaseAuthenticationSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, FirebaseAuthenticationSettings::_Default)
    }
}

impl AsRef<FirebaseAuthenticationSettings> for FirebaseAuthenticationSettings {
    fn as_ref(&self) -> &FirebaseAuthenticationSettings {
        self
    }
}

/// Settings for Firebase Authentication in the official Android application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FirebaseAuthenticationSettingsAndroid {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FirebaseAuthenticationSettingsAndroid {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDFirebaseAuthenticationSettings for FirebaseAuthenticationSettingsAndroid {}

impl FirebaseAuthenticationSettingsAndroid {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FirebaseAuthenticationSettingsAndroidBuilder {
        let mut inner = FirebaseAuthenticationSettingsAndroid::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FirebaseAuthenticationSettingsAndroidBuilder { inner }
    }
}

#[doc(hidden)]
pub struct FirebaseAuthenticationSettingsAndroidBuilder {
    inner: FirebaseAuthenticationSettingsAndroid,
}

#[deprecated]
pub type RTDFirebaseAuthenticationSettingsAndroidBuilder =
    FirebaseAuthenticationSettingsAndroidBuilder;

impl FirebaseAuthenticationSettingsAndroidBuilder {
    pub fn build(&self) -> FirebaseAuthenticationSettingsAndroid {
        self.inner.clone()
    }
}

impl AsRef<FirebaseAuthenticationSettingsAndroid> for FirebaseAuthenticationSettingsAndroid {
    fn as_ref(&self) -> &FirebaseAuthenticationSettingsAndroid {
        self
    }
}

impl AsRef<FirebaseAuthenticationSettingsAndroid> for FirebaseAuthenticationSettingsAndroidBuilder {
    fn as_ref(&self) -> &FirebaseAuthenticationSettingsAndroid {
        &self.inner
    }
}

/// Settings for Firebase Authentication in the official iOS application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FirebaseAuthenticationSettingsIos {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Device token from Apple Push Notification service

    #[serde(default)]
    device_token: String,
    /// True, if App Sandbox is enabled

    #[serde(default)]
    is_app_sandbox: bool,
}

impl RObject for FirebaseAuthenticationSettingsIos {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDFirebaseAuthenticationSettings for FirebaseAuthenticationSettingsIos {}

impl FirebaseAuthenticationSettingsIos {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FirebaseAuthenticationSettingsIosBuilder {
        let mut inner = FirebaseAuthenticationSettingsIos::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FirebaseAuthenticationSettingsIosBuilder { inner }
    }

    pub fn device_token(&self) -> &String {
        &self.device_token
    }

    pub fn is_app_sandbox(&self) -> bool {
        self.is_app_sandbox
    }
}

#[doc(hidden)]
pub struct FirebaseAuthenticationSettingsIosBuilder {
    inner: FirebaseAuthenticationSettingsIos,
}

#[deprecated]
pub type RTDFirebaseAuthenticationSettingsIosBuilder = FirebaseAuthenticationSettingsIosBuilder;

impl FirebaseAuthenticationSettingsIosBuilder {
    pub fn build(&self) -> FirebaseAuthenticationSettingsIos {
        self.inner.clone()
    }

    pub fn device_token<T: AsRef<str>>(&mut self, device_token: T) -> &mut Self {
        self.inner.device_token = device_token.as_ref().to_string();
        self
    }

    pub fn is_app_sandbox(&mut self, is_app_sandbox: bool) -> &mut Self {
        self.inner.is_app_sandbox = is_app_sandbox;
        self
    }
}

impl AsRef<FirebaseAuthenticationSettingsIos> for FirebaseAuthenticationSettingsIos {
    fn as_ref(&self) -> &FirebaseAuthenticationSettingsIos {
        self
    }
}

impl AsRef<FirebaseAuthenticationSettingsIos> for FirebaseAuthenticationSettingsIosBuilder {
    fn as_ref(&self) -> &FirebaseAuthenticationSettingsIos {
        &self.inner
    }
}
