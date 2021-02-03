use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Represents a data needed to subscribe for push notifications through registerDevice method. To use specific push notification service, the correct application platform must be specified and a valid server authentication data must be uploaded at https://my.telegram.org
pub trait TDDeviceToken: Debug + RObject {}

/// Represents a data needed to subscribe for push notifications through registerDevice method. To use specific push notification service, the correct application platform must be specified and a valid server authentication data must be uploaded at https://my.telegram.org
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum DeviceToken {
    #[doc(hidden)]
    _Default(()),
    /// A token for Apple Push Notification service
    ApplePush(DeviceTokenApplePush),
    /// A token for Apple Push Notification service VoIP notifications
    ApplePushVoIP(DeviceTokenApplePushVoIP),
    /// A token for BlackBerry Push Service
    BlackBerryPush(DeviceTokenBlackBerryPush),
    /// A token for Firebase Cloud Messaging
    FirebaseCloudMessaging(DeviceTokenFirebaseCloudMessaging),
    /// A token for Microsoft Push Notification Service
    MicrosoftPush(DeviceTokenMicrosoftPush),
    /// A token for Microsoft Push Notification Service VoIP channel
    MicrosoftPushVoIP(DeviceTokenMicrosoftPushVoIP),
    /// A token for Simple Push API for Firefox OS
    SimplePush(DeviceTokenSimplePush),
    /// A token for Tizen Push Service
    TizenPush(DeviceTokenTizenPush),
    /// A token for Ubuntu Push Client service
    UbuntuPush(DeviceTokenUbuntuPush),
    /// A token for web Push API
    WebPush(DeviceTokenWebPush),
    /// A token for Windows Push Notification Services
    WindowsPush(DeviceTokenWindowsPush),
}

impl Default for DeviceToken {
    fn default() -> Self {
        DeviceToken::_Default(())
    }
}

impl<'de> Deserialize<'de> for DeviceToken {
    fn deserialize<D>(deserializer: D) -> Result<DeviceToken, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          DeviceToken,
          (deviceTokenApplePush, ApplePush);
          (deviceTokenApplePushVoIP, ApplePushVoIP);
          (deviceTokenBlackBerryPush, BlackBerryPush);
          (deviceTokenFirebaseCloudMessaging, FirebaseCloudMessaging);
          (deviceTokenMicrosoftPush, MicrosoftPush);
          (deviceTokenMicrosoftPushVoIP, MicrosoftPushVoIP);
          (deviceTokenSimplePush, SimplePush);
          (deviceTokenTizenPush, TizenPush);
          (deviceTokenUbuntuPush, UbuntuPush);
          (deviceTokenWebPush, WebPush);
          (deviceTokenWindowsPush, WindowsPush);

        )(deserializer)
    }
}

impl RObject for DeviceToken {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            DeviceToken::ApplePush(t) => t.td_name(),
            DeviceToken::ApplePushVoIP(t) => t.td_name(),
            DeviceToken::BlackBerryPush(t) => t.td_name(),
            DeviceToken::FirebaseCloudMessaging(t) => t.td_name(),
            DeviceToken::MicrosoftPush(t) => t.td_name(),
            DeviceToken::MicrosoftPushVoIP(t) => t.td_name(),
            DeviceToken::SimplePush(t) => t.td_name(),
            DeviceToken::TizenPush(t) => t.td_name(),
            DeviceToken::UbuntuPush(t) => t.td_name(),
            DeviceToken::WebPush(t) => t.td_name(),
            DeviceToken::WindowsPush(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            DeviceToken::ApplePush(t) => t.extra(),
            DeviceToken::ApplePushVoIP(t) => t.extra(),
            DeviceToken::BlackBerryPush(t) => t.extra(),
            DeviceToken::FirebaseCloudMessaging(t) => t.extra(),
            DeviceToken::MicrosoftPush(t) => t.extra(),
            DeviceToken::MicrosoftPushVoIP(t) => t.extra(),
            DeviceToken::SimplePush(t) => t.extra(),
            DeviceToken::TizenPush(t) => t.extra(),
            DeviceToken::UbuntuPush(t) => t.extra(),
            DeviceToken::WebPush(t) => t.extra(),
            DeviceToken::WindowsPush(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            DeviceToken::ApplePush(t) => t.client_id(),
            DeviceToken::ApplePushVoIP(t) => t.client_id(),
            DeviceToken::BlackBerryPush(t) => t.client_id(),
            DeviceToken::FirebaseCloudMessaging(t) => t.client_id(),
            DeviceToken::MicrosoftPush(t) => t.client_id(),
            DeviceToken::MicrosoftPushVoIP(t) => t.client_id(),
            DeviceToken::SimplePush(t) => t.client_id(),
            DeviceToken::TizenPush(t) => t.client_id(),
            DeviceToken::UbuntuPush(t) => t.client_id(),
            DeviceToken::WebPush(t) => t.client_id(),
            DeviceToken::WindowsPush(t) => t.client_id(),

            _ => None,
        }
    }
}

impl DeviceToken {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, DeviceToken::_Default(_))
    }
}

impl AsRef<DeviceToken> for DeviceToken {
    fn as_ref(&self) -> &DeviceToken {
        self
    }
}

/// A token for Apple Push Notification service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenApplePush {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Device token; may be empty to de-register a device
    device_token: String,
    /// True, if App Sandbox is enabled
    is_app_sandbox: bool,
}

impl RObject for DeviceTokenApplePush {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "deviceTokenApplePush"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDDeviceToken for DeviceTokenApplePush {}

impl DeviceTokenApplePush {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeviceTokenApplePushBuilder {
        let mut inner = DeviceTokenApplePush::default();
        inner.td_name = "deviceTokenApplePush".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDDeviceTokenApplePushBuilder { inner }
    }

    pub fn device_token(&self) -> &String {
        &self.device_token
    }

    pub fn is_app_sandbox(&self) -> bool {
        self.is_app_sandbox
    }
}

#[doc(hidden)]
pub struct RTDDeviceTokenApplePushBuilder {
    inner: DeviceTokenApplePush,
}

impl RTDDeviceTokenApplePushBuilder {
    pub fn build(&self) -> DeviceTokenApplePush {
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

impl AsRef<DeviceTokenApplePush> for DeviceTokenApplePush {
    fn as_ref(&self) -> &DeviceTokenApplePush {
        self
    }
}

impl AsRef<DeviceTokenApplePush> for RTDDeviceTokenApplePushBuilder {
    fn as_ref(&self) -> &DeviceTokenApplePush {
        &self.inner
    }
}

/// A token for Apple Push Notification service VoIP notifications
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenApplePushVoIP {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Device token; may be empty to de-register a device
    device_token: String,
    /// True, if App Sandbox is enabled
    is_app_sandbox: bool,
    /// True, if push notifications should be additionally encrypted
    encrypt: bool,
}

impl RObject for DeviceTokenApplePushVoIP {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "deviceTokenApplePushVoIP"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDDeviceToken for DeviceTokenApplePushVoIP {}

impl DeviceTokenApplePushVoIP {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeviceTokenApplePushVoIPBuilder {
        let mut inner = DeviceTokenApplePushVoIP::default();
        inner.td_name = "deviceTokenApplePushVoIP".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDDeviceTokenApplePushVoIPBuilder { inner }
    }

    pub fn device_token(&self) -> &String {
        &self.device_token
    }

    pub fn is_app_sandbox(&self) -> bool {
        self.is_app_sandbox
    }

    pub fn encrypt(&self) -> bool {
        self.encrypt
    }
}

#[doc(hidden)]
pub struct RTDDeviceTokenApplePushVoIPBuilder {
    inner: DeviceTokenApplePushVoIP,
}

impl RTDDeviceTokenApplePushVoIPBuilder {
    pub fn build(&self) -> DeviceTokenApplePushVoIP {
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

    pub fn encrypt(&mut self, encrypt: bool) -> &mut Self {
        self.inner.encrypt = encrypt;
        self
    }
}

impl AsRef<DeviceTokenApplePushVoIP> for DeviceTokenApplePushVoIP {
    fn as_ref(&self) -> &DeviceTokenApplePushVoIP {
        self
    }
}

impl AsRef<DeviceTokenApplePushVoIP> for RTDDeviceTokenApplePushVoIPBuilder {
    fn as_ref(&self) -> &DeviceTokenApplePushVoIP {
        &self.inner
    }
}

/// A token for BlackBerry Push Service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenBlackBerryPush {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Token; may be empty to de-register a device
    token: String,
}

impl RObject for DeviceTokenBlackBerryPush {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "deviceTokenBlackBerryPush"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDDeviceToken for DeviceTokenBlackBerryPush {}

impl DeviceTokenBlackBerryPush {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeviceTokenBlackBerryPushBuilder {
        let mut inner = DeviceTokenBlackBerryPush::default();
        inner.td_name = "deviceTokenBlackBerryPush".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDDeviceTokenBlackBerryPushBuilder { inner }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

#[doc(hidden)]
pub struct RTDDeviceTokenBlackBerryPushBuilder {
    inner: DeviceTokenBlackBerryPush,
}

impl RTDDeviceTokenBlackBerryPushBuilder {
    pub fn build(&self) -> DeviceTokenBlackBerryPush {
        self.inner.clone()
    }

    pub fn token<T: AsRef<str>>(&mut self, token: T) -> &mut Self {
        self.inner.token = token.as_ref().to_string();
        self
    }
}

impl AsRef<DeviceTokenBlackBerryPush> for DeviceTokenBlackBerryPush {
    fn as_ref(&self) -> &DeviceTokenBlackBerryPush {
        self
    }
}

impl AsRef<DeviceTokenBlackBerryPush> for RTDDeviceTokenBlackBerryPushBuilder {
    fn as_ref(&self) -> &DeviceTokenBlackBerryPush {
        &self.inner
    }
}

/// A token for Firebase Cloud Messaging
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenFirebaseCloudMessaging {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Device registration token; may be empty to de-register a device
    token: String,
    /// True, if push notifications should be additionally encrypted
    encrypt: bool,
}

impl RObject for DeviceTokenFirebaseCloudMessaging {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "deviceTokenFirebaseCloudMessaging"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDDeviceToken for DeviceTokenFirebaseCloudMessaging {}

impl DeviceTokenFirebaseCloudMessaging {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeviceTokenFirebaseCloudMessagingBuilder {
        let mut inner = DeviceTokenFirebaseCloudMessaging::default();
        inner.td_name = "deviceTokenFirebaseCloudMessaging".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDDeviceTokenFirebaseCloudMessagingBuilder { inner }
    }

    pub fn token(&self) -> &String {
        &self.token
    }

    pub fn encrypt(&self) -> bool {
        self.encrypt
    }
}

#[doc(hidden)]
pub struct RTDDeviceTokenFirebaseCloudMessagingBuilder {
    inner: DeviceTokenFirebaseCloudMessaging,
}

impl RTDDeviceTokenFirebaseCloudMessagingBuilder {
    pub fn build(&self) -> DeviceTokenFirebaseCloudMessaging {
        self.inner.clone()
    }

    pub fn token<T: AsRef<str>>(&mut self, token: T) -> &mut Self {
        self.inner.token = token.as_ref().to_string();
        self
    }

    pub fn encrypt(&mut self, encrypt: bool) -> &mut Self {
        self.inner.encrypt = encrypt;
        self
    }
}

impl AsRef<DeviceTokenFirebaseCloudMessaging> for DeviceTokenFirebaseCloudMessaging {
    fn as_ref(&self) -> &DeviceTokenFirebaseCloudMessaging {
        self
    }
}

impl AsRef<DeviceTokenFirebaseCloudMessaging> for RTDDeviceTokenFirebaseCloudMessagingBuilder {
    fn as_ref(&self) -> &DeviceTokenFirebaseCloudMessaging {
        &self.inner
    }
}

/// A token for Microsoft Push Notification Service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenMicrosoftPush {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Push notification channel URI; may be empty to de-register a device
    channel_uri: String,
}

impl RObject for DeviceTokenMicrosoftPush {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "deviceTokenMicrosoftPush"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDDeviceToken for DeviceTokenMicrosoftPush {}

impl DeviceTokenMicrosoftPush {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeviceTokenMicrosoftPushBuilder {
        let mut inner = DeviceTokenMicrosoftPush::default();
        inner.td_name = "deviceTokenMicrosoftPush".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDDeviceTokenMicrosoftPushBuilder { inner }
    }

    pub fn channel_uri(&self) -> &String {
        &self.channel_uri
    }
}

#[doc(hidden)]
pub struct RTDDeviceTokenMicrosoftPushBuilder {
    inner: DeviceTokenMicrosoftPush,
}

impl RTDDeviceTokenMicrosoftPushBuilder {
    pub fn build(&self) -> DeviceTokenMicrosoftPush {
        self.inner.clone()
    }

    pub fn channel_uri<T: AsRef<str>>(&mut self, channel_uri: T) -> &mut Self {
        self.inner.channel_uri = channel_uri.as_ref().to_string();
        self
    }
}

impl AsRef<DeviceTokenMicrosoftPush> for DeviceTokenMicrosoftPush {
    fn as_ref(&self) -> &DeviceTokenMicrosoftPush {
        self
    }
}

impl AsRef<DeviceTokenMicrosoftPush> for RTDDeviceTokenMicrosoftPushBuilder {
    fn as_ref(&self) -> &DeviceTokenMicrosoftPush {
        &self.inner
    }
}

/// A token for Microsoft Push Notification Service VoIP channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenMicrosoftPushVoIP {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Push notification channel URI; may be empty to de-register a device
    channel_uri: String,
}

impl RObject for DeviceTokenMicrosoftPushVoIP {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "deviceTokenMicrosoftPushVoIP"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDDeviceToken for DeviceTokenMicrosoftPushVoIP {}

impl DeviceTokenMicrosoftPushVoIP {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeviceTokenMicrosoftPushVoIPBuilder {
        let mut inner = DeviceTokenMicrosoftPushVoIP::default();
        inner.td_name = "deviceTokenMicrosoftPushVoIP".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDDeviceTokenMicrosoftPushVoIPBuilder { inner }
    }

    pub fn channel_uri(&self) -> &String {
        &self.channel_uri
    }
}

#[doc(hidden)]
pub struct RTDDeviceTokenMicrosoftPushVoIPBuilder {
    inner: DeviceTokenMicrosoftPushVoIP,
}

impl RTDDeviceTokenMicrosoftPushVoIPBuilder {
    pub fn build(&self) -> DeviceTokenMicrosoftPushVoIP {
        self.inner.clone()
    }

    pub fn channel_uri<T: AsRef<str>>(&mut self, channel_uri: T) -> &mut Self {
        self.inner.channel_uri = channel_uri.as_ref().to_string();
        self
    }
}

impl AsRef<DeviceTokenMicrosoftPushVoIP> for DeviceTokenMicrosoftPushVoIP {
    fn as_ref(&self) -> &DeviceTokenMicrosoftPushVoIP {
        self
    }
}

impl AsRef<DeviceTokenMicrosoftPushVoIP> for RTDDeviceTokenMicrosoftPushVoIPBuilder {
    fn as_ref(&self) -> &DeviceTokenMicrosoftPushVoIP {
        &self.inner
    }
}

/// A token for Simple Push API for Firefox OS
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenSimplePush {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Absolute URL exposed by the push service where the application server can send push messages; may be empty to de-register a device
    endpoint: String,
}

impl RObject for DeviceTokenSimplePush {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "deviceTokenSimplePush"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDDeviceToken for DeviceTokenSimplePush {}

impl DeviceTokenSimplePush {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeviceTokenSimplePushBuilder {
        let mut inner = DeviceTokenSimplePush::default();
        inner.td_name = "deviceTokenSimplePush".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDDeviceTokenSimplePushBuilder { inner }
    }

    pub fn endpoint(&self) -> &String {
        &self.endpoint
    }
}

#[doc(hidden)]
pub struct RTDDeviceTokenSimplePushBuilder {
    inner: DeviceTokenSimplePush,
}

impl RTDDeviceTokenSimplePushBuilder {
    pub fn build(&self) -> DeviceTokenSimplePush {
        self.inner.clone()
    }

    pub fn endpoint<T: AsRef<str>>(&mut self, endpoint: T) -> &mut Self {
        self.inner.endpoint = endpoint.as_ref().to_string();
        self
    }
}

impl AsRef<DeviceTokenSimplePush> for DeviceTokenSimplePush {
    fn as_ref(&self) -> &DeviceTokenSimplePush {
        self
    }
}

impl AsRef<DeviceTokenSimplePush> for RTDDeviceTokenSimplePushBuilder {
    fn as_ref(&self) -> &DeviceTokenSimplePush {
        &self.inner
    }
}

/// A token for Tizen Push Service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenTizenPush {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Push service registration identifier; may be empty to de-register a device
    reg_id: String,
}

impl RObject for DeviceTokenTizenPush {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "deviceTokenTizenPush"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDDeviceToken for DeviceTokenTizenPush {}

impl DeviceTokenTizenPush {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeviceTokenTizenPushBuilder {
        let mut inner = DeviceTokenTizenPush::default();
        inner.td_name = "deviceTokenTizenPush".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDDeviceTokenTizenPushBuilder { inner }
    }

    pub fn reg_id(&self) -> &String {
        &self.reg_id
    }
}

#[doc(hidden)]
pub struct RTDDeviceTokenTizenPushBuilder {
    inner: DeviceTokenTizenPush,
}

impl RTDDeviceTokenTizenPushBuilder {
    pub fn build(&self) -> DeviceTokenTizenPush {
        self.inner.clone()
    }

    pub fn reg_id<T: AsRef<str>>(&mut self, reg_id: T) -> &mut Self {
        self.inner.reg_id = reg_id.as_ref().to_string();
        self
    }
}

impl AsRef<DeviceTokenTizenPush> for DeviceTokenTizenPush {
    fn as_ref(&self) -> &DeviceTokenTizenPush {
        self
    }
}

impl AsRef<DeviceTokenTizenPush> for RTDDeviceTokenTizenPushBuilder {
    fn as_ref(&self) -> &DeviceTokenTizenPush {
        &self.inner
    }
}

/// A token for Ubuntu Push Client service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenUbuntuPush {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Token; may be empty to de-register a device
    token: String,
}

impl RObject for DeviceTokenUbuntuPush {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "deviceTokenUbuntuPush"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDDeviceToken for DeviceTokenUbuntuPush {}

impl DeviceTokenUbuntuPush {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeviceTokenUbuntuPushBuilder {
        let mut inner = DeviceTokenUbuntuPush::default();
        inner.td_name = "deviceTokenUbuntuPush".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDDeviceTokenUbuntuPushBuilder { inner }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

#[doc(hidden)]
pub struct RTDDeviceTokenUbuntuPushBuilder {
    inner: DeviceTokenUbuntuPush,
}

impl RTDDeviceTokenUbuntuPushBuilder {
    pub fn build(&self) -> DeviceTokenUbuntuPush {
        self.inner.clone()
    }

    pub fn token<T: AsRef<str>>(&mut self, token: T) -> &mut Self {
        self.inner.token = token.as_ref().to_string();
        self
    }
}

impl AsRef<DeviceTokenUbuntuPush> for DeviceTokenUbuntuPush {
    fn as_ref(&self) -> &DeviceTokenUbuntuPush {
        self
    }
}

impl AsRef<DeviceTokenUbuntuPush> for RTDDeviceTokenUbuntuPushBuilder {
    fn as_ref(&self) -> &DeviceTokenUbuntuPush {
        &self.inner
    }
}

/// A token for web Push API
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenWebPush {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Absolute URL exposed by the push service where the application server can send push messages; may be empty to de-register a device
    endpoint: String,
    /// Base64url-encoded P-256 elliptic curve Diffie-Hellman public key
    p256dh_base64url: String,
    /// Base64url-encoded authentication secret
    auth_base64url: String,
}

impl RObject for DeviceTokenWebPush {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "deviceTokenWebPush"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDDeviceToken for DeviceTokenWebPush {}

impl DeviceTokenWebPush {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeviceTokenWebPushBuilder {
        let mut inner = DeviceTokenWebPush::default();
        inner.td_name = "deviceTokenWebPush".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDDeviceTokenWebPushBuilder { inner }
    }

    pub fn endpoint(&self) -> &String {
        &self.endpoint
    }

    pub fn p256dh_base64url(&self) -> &String {
        &self.p256dh_base64url
    }

    pub fn auth_base64url(&self) -> &String {
        &self.auth_base64url
    }
}

#[doc(hidden)]
pub struct RTDDeviceTokenWebPushBuilder {
    inner: DeviceTokenWebPush,
}

impl RTDDeviceTokenWebPushBuilder {
    pub fn build(&self) -> DeviceTokenWebPush {
        self.inner.clone()
    }

    pub fn endpoint<T: AsRef<str>>(&mut self, endpoint: T) -> &mut Self {
        self.inner.endpoint = endpoint.as_ref().to_string();
        self
    }

    pub fn p256dh_base64url<T: AsRef<str>>(&mut self, p256dh_base64url: T) -> &mut Self {
        self.inner.p256dh_base64url = p256dh_base64url.as_ref().to_string();
        self
    }

    pub fn auth_base64url<T: AsRef<str>>(&mut self, auth_base64url: T) -> &mut Self {
        self.inner.auth_base64url = auth_base64url.as_ref().to_string();
        self
    }
}

impl AsRef<DeviceTokenWebPush> for DeviceTokenWebPush {
    fn as_ref(&self) -> &DeviceTokenWebPush {
        self
    }
}

impl AsRef<DeviceTokenWebPush> for RTDDeviceTokenWebPushBuilder {
    fn as_ref(&self) -> &DeviceTokenWebPush {
        &self.inner
    }
}

/// A token for Windows Push Notification Services
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenWindowsPush {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The access token that will be used to send notifications; may be empty to de-register a device
    access_token: String,
}

impl RObject for DeviceTokenWindowsPush {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "deviceTokenWindowsPush"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDDeviceToken for DeviceTokenWindowsPush {}

impl DeviceTokenWindowsPush {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeviceTokenWindowsPushBuilder {
        let mut inner = DeviceTokenWindowsPush::default();
        inner.td_name = "deviceTokenWindowsPush".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDDeviceTokenWindowsPushBuilder { inner }
    }

    pub fn access_token(&self) -> &String {
        &self.access_token
    }
}

#[doc(hidden)]
pub struct RTDDeviceTokenWindowsPushBuilder {
    inner: DeviceTokenWindowsPush,
}

impl RTDDeviceTokenWindowsPushBuilder {
    pub fn build(&self) -> DeviceTokenWindowsPush {
        self.inner.clone()
    }

    pub fn access_token<T: AsRef<str>>(&mut self, access_token: T) -> &mut Self {
        self.inner.access_token = access_token.as_ref().to_string();
        self
    }
}

impl AsRef<DeviceTokenWindowsPush> for DeviceTokenWindowsPush {
    fn as_ref(&self) -> &DeviceTokenWindowsPush {
        self
    }
}

impl AsRef<DeviceTokenWindowsPush> for RTDDeviceTokenWindowsPushBuilder {
    fn as_ref(&self) -> &DeviceTokenWindowsPush {
        &self.inner
    }
}
