use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Represents a data needed to subscribe for push notifications through registerDevice method. To use specific push notification service, the correct application platform must be specified and a valid server authentication data must be uploaded at https://my.telegram.org
pub trait TDDeviceToken: Debug + RObject {}

/// Represents a data needed to subscribe for push notifications through registerDevice method. To use specific push notification service, the correct application platform must be specified and a valid server authentication data must be uploaded at https://my.telegram.org
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum DeviceToken {
    #[doc(hidden)]
    _Default,
    /// A token for Apple Push Notification service
    #[serde(rename = "deviceTokenApplePush")]
    ApplePush(DeviceTokenApplePush),
    /// A token for Apple Push Notification service VoIP notifications
    #[serde(rename = "deviceTokenApplePushVoIP")]
    ApplePushVoIP(DeviceTokenApplePushVoIP),
    /// A token for BlackBerry Push Service
    #[serde(rename = "deviceTokenBlackBerryPush")]
    BlackBerryPush(DeviceTokenBlackBerryPush),
    /// A token for Firebase Cloud Messaging
    #[serde(rename = "deviceTokenFirebaseCloudMessaging")]
    FirebaseCloudMessaging(DeviceTokenFirebaseCloudMessaging),
    /// A token for Microsoft Push Notification Service
    #[serde(rename = "deviceTokenMicrosoftPush")]
    MicrosoftPush(DeviceTokenMicrosoftPush),
    /// A token for Microsoft Push Notification Service VoIP channel
    #[serde(rename = "deviceTokenMicrosoftPushVoIP")]
    MicrosoftPushVoIP(DeviceTokenMicrosoftPushVoIP),
    /// A token for Simple Push API for Firefox OS
    #[serde(rename = "deviceTokenSimplePush")]
    SimplePush(DeviceTokenSimplePush),
    /// A token for Tizen Push Service
    #[serde(rename = "deviceTokenTizenPush")]
    TizenPush(DeviceTokenTizenPush),
    /// A token for Ubuntu Push Client service
    #[serde(rename = "deviceTokenUbuntuPush")]
    UbuntuPush(DeviceTokenUbuntuPush),
    /// A token for web Push API
    #[serde(rename = "deviceTokenWebPush")]
    WebPush(DeviceTokenWebPush),
    /// A token for Windows Push Notification Services
    #[serde(rename = "deviceTokenWindowsPush")]
    WindowsPush(DeviceTokenWindowsPush),
}

impl Default for DeviceToken {
    fn default() -> Self {
        DeviceToken::_Default
    }
}

impl RObject for DeviceToken {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, DeviceToken::_Default)
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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Device token; may be empty to deregister a device

    #[serde(default)]
    device_token: String,
    /// True, if App Sandbox is enabled

    #[serde(default)]
    is_app_sandbox: bool,
}

impl RObject for DeviceTokenApplePush {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDDeviceToken for DeviceTokenApplePush {}

impl DeviceTokenApplePush {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DeviceTokenApplePushBuilder {
        let mut inner = DeviceTokenApplePush::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        DeviceTokenApplePushBuilder { inner }
    }

    pub fn device_token(&self) -> &String {
        &self.device_token
    }

    pub fn is_app_sandbox(&self) -> bool {
        self.is_app_sandbox
    }
}

#[doc(hidden)]
pub struct DeviceTokenApplePushBuilder {
    inner: DeviceTokenApplePush,
}

#[deprecated]
pub type RTDDeviceTokenApplePushBuilder = DeviceTokenApplePushBuilder;

impl DeviceTokenApplePushBuilder {
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

impl AsRef<DeviceTokenApplePush> for DeviceTokenApplePushBuilder {
    fn as_ref(&self) -> &DeviceTokenApplePush {
        &self.inner
    }
}

/// A token for Apple Push Notification service VoIP notifications
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenApplePushVoIP {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Device token; may be empty to deregister a device

    #[serde(default)]
    device_token: String,
    /// True, if App Sandbox is enabled

    #[serde(default)]
    is_app_sandbox: bool,
    /// True, if push notifications must be additionally encrypted

    #[serde(default)]
    encrypt: bool,
}

impl RObject for DeviceTokenApplePushVoIP {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDDeviceToken for DeviceTokenApplePushVoIP {}

impl DeviceTokenApplePushVoIP {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DeviceTokenApplePushVoIPBuilder {
        let mut inner = DeviceTokenApplePushVoIP::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        DeviceTokenApplePushVoIPBuilder { inner }
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
pub struct DeviceTokenApplePushVoIPBuilder {
    inner: DeviceTokenApplePushVoIP,
}

#[deprecated]
pub type RTDDeviceTokenApplePushVoIPBuilder = DeviceTokenApplePushVoIPBuilder;

impl DeviceTokenApplePushVoIPBuilder {
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

impl AsRef<DeviceTokenApplePushVoIP> for DeviceTokenApplePushVoIPBuilder {
    fn as_ref(&self) -> &DeviceTokenApplePushVoIP {
        &self.inner
    }
}

/// A token for BlackBerry Push Service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenBlackBerryPush {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Token; may be empty to deregister a device

    #[serde(default)]
    token: String,
}

impl RObject for DeviceTokenBlackBerryPush {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDDeviceToken for DeviceTokenBlackBerryPush {}

impl DeviceTokenBlackBerryPush {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DeviceTokenBlackBerryPushBuilder {
        let mut inner = DeviceTokenBlackBerryPush::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        DeviceTokenBlackBerryPushBuilder { inner }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

#[doc(hidden)]
pub struct DeviceTokenBlackBerryPushBuilder {
    inner: DeviceTokenBlackBerryPush,
}

#[deprecated]
pub type RTDDeviceTokenBlackBerryPushBuilder = DeviceTokenBlackBerryPushBuilder;

impl DeviceTokenBlackBerryPushBuilder {
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

impl AsRef<DeviceTokenBlackBerryPush> for DeviceTokenBlackBerryPushBuilder {
    fn as_ref(&self) -> &DeviceTokenBlackBerryPush {
        &self.inner
    }
}

/// A token for Firebase Cloud Messaging
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenFirebaseCloudMessaging {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Device registration token; may be empty to deregister a device

    #[serde(default)]
    token: String,
    /// True, if push notifications must be additionally encrypted

    #[serde(default)]
    encrypt: bool,
}

impl RObject for DeviceTokenFirebaseCloudMessaging {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDDeviceToken for DeviceTokenFirebaseCloudMessaging {}

impl DeviceTokenFirebaseCloudMessaging {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DeviceTokenFirebaseCloudMessagingBuilder {
        let mut inner = DeviceTokenFirebaseCloudMessaging::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        DeviceTokenFirebaseCloudMessagingBuilder { inner }
    }

    pub fn token(&self) -> &String {
        &self.token
    }

    pub fn encrypt(&self) -> bool {
        self.encrypt
    }
}

#[doc(hidden)]
pub struct DeviceTokenFirebaseCloudMessagingBuilder {
    inner: DeviceTokenFirebaseCloudMessaging,
}

#[deprecated]
pub type RTDDeviceTokenFirebaseCloudMessagingBuilder = DeviceTokenFirebaseCloudMessagingBuilder;

impl DeviceTokenFirebaseCloudMessagingBuilder {
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

impl AsRef<DeviceTokenFirebaseCloudMessaging> for DeviceTokenFirebaseCloudMessagingBuilder {
    fn as_ref(&self) -> &DeviceTokenFirebaseCloudMessaging {
        &self.inner
    }
}

/// A token for Microsoft Push Notification Service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenMicrosoftPush {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Push notification channel URI; may be empty to deregister a device

    #[serde(default)]
    channel_uri: String,
}

impl RObject for DeviceTokenMicrosoftPush {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDDeviceToken for DeviceTokenMicrosoftPush {}

impl DeviceTokenMicrosoftPush {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DeviceTokenMicrosoftPushBuilder {
        let mut inner = DeviceTokenMicrosoftPush::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        DeviceTokenMicrosoftPushBuilder { inner }
    }

    pub fn channel_uri(&self) -> &String {
        &self.channel_uri
    }
}

#[doc(hidden)]
pub struct DeviceTokenMicrosoftPushBuilder {
    inner: DeviceTokenMicrosoftPush,
}

#[deprecated]
pub type RTDDeviceTokenMicrosoftPushBuilder = DeviceTokenMicrosoftPushBuilder;

impl DeviceTokenMicrosoftPushBuilder {
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

impl AsRef<DeviceTokenMicrosoftPush> for DeviceTokenMicrosoftPushBuilder {
    fn as_ref(&self) -> &DeviceTokenMicrosoftPush {
        &self.inner
    }
}

/// A token for Microsoft Push Notification Service VoIP channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenMicrosoftPushVoIP {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Push notification channel URI; may be empty to deregister a device

    #[serde(default)]
    channel_uri: String,
}

impl RObject for DeviceTokenMicrosoftPushVoIP {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDDeviceToken for DeviceTokenMicrosoftPushVoIP {}

impl DeviceTokenMicrosoftPushVoIP {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DeviceTokenMicrosoftPushVoIPBuilder {
        let mut inner = DeviceTokenMicrosoftPushVoIP::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        DeviceTokenMicrosoftPushVoIPBuilder { inner }
    }

    pub fn channel_uri(&self) -> &String {
        &self.channel_uri
    }
}

#[doc(hidden)]
pub struct DeviceTokenMicrosoftPushVoIPBuilder {
    inner: DeviceTokenMicrosoftPushVoIP,
}

#[deprecated]
pub type RTDDeviceTokenMicrosoftPushVoIPBuilder = DeviceTokenMicrosoftPushVoIPBuilder;

impl DeviceTokenMicrosoftPushVoIPBuilder {
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

impl AsRef<DeviceTokenMicrosoftPushVoIP> for DeviceTokenMicrosoftPushVoIPBuilder {
    fn as_ref(&self) -> &DeviceTokenMicrosoftPushVoIP {
        &self.inner
    }
}

/// A token for Simple Push API for Firefox OS
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenSimplePush {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Absolute URL exposed by the push service where the application server can send push messages; may be empty to deregister a device

    #[serde(default)]
    endpoint: String,
}

impl RObject for DeviceTokenSimplePush {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDDeviceToken for DeviceTokenSimplePush {}

impl DeviceTokenSimplePush {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DeviceTokenSimplePushBuilder {
        let mut inner = DeviceTokenSimplePush::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        DeviceTokenSimplePushBuilder { inner }
    }

    pub fn endpoint(&self) -> &String {
        &self.endpoint
    }
}

#[doc(hidden)]
pub struct DeviceTokenSimplePushBuilder {
    inner: DeviceTokenSimplePush,
}

#[deprecated]
pub type RTDDeviceTokenSimplePushBuilder = DeviceTokenSimplePushBuilder;

impl DeviceTokenSimplePushBuilder {
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

impl AsRef<DeviceTokenSimplePush> for DeviceTokenSimplePushBuilder {
    fn as_ref(&self) -> &DeviceTokenSimplePush {
        &self.inner
    }
}

/// A token for Tizen Push Service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenTizenPush {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Push service registration identifier; may be empty to deregister a device

    #[serde(default)]
    reg_id: String,
}

impl RObject for DeviceTokenTizenPush {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDDeviceToken for DeviceTokenTizenPush {}

impl DeviceTokenTizenPush {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DeviceTokenTizenPushBuilder {
        let mut inner = DeviceTokenTizenPush::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        DeviceTokenTizenPushBuilder { inner }
    }

    pub fn reg_id(&self) -> &String {
        &self.reg_id
    }
}

#[doc(hidden)]
pub struct DeviceTokenTizenPushBuilder {
    inner: DeviceTokenTizenPush,
}

#[deprecated]
pub type RTDDeviceTokenTizenPushBuilder = DeviceTokenTizenPushBuilder;

impl DeviceTokenTizenPushBuilder {
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

impl AsRef<DeviceTokenTizenPush> for DeviceTokenTizenPushBuilder {
    fn as_ref(&self) -> &DeviceTokenTizenPush {
        &self.inner
    }
}

/// A token for Ubuntu Push Client service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenUbuntuPush {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Token; may be empty to deregister a device

    #[serde(default)]
    token: String,
}

impl RObject for DeviceTokenUbuntuPush {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDDeviceToken for DeviceTokenUbuntuPush {}

impl DeviceTokenUbuntuPush {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DeviceTokenUbuntuPushBuilder {
        let mut inner = DeviceTokenUbuntuPush::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        DeviceTokenUbuntuPushBuilder { inner }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

#[doc(hidden)]
pub struct DeviceTokenUbuntuPushBuilder {
    inner: DeviceTokenUbuntuPush,
}

#[deprecated]
pub type RTDDeviceTokenUbuntuPushBuilder = DeviceTokenUbuntuPushBuilder;

impl DeviceTokenUbuntuPushBuilder {
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

impl AsRef<DeviceTokenUbuntuPush> for DeviceTokenUbuntuPushBuilder {
    fn as_ref(&self) -> &DeviceTokenUbuntuPush {
        &self.inner
    }
}

/// A token for web Push API
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenWebPush {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Absolute URL exposed by the push service where the application server can send push messages; may be empty to deregister a device

    #[serde(default)]
    endpoint: String,
    /// Base64url-encoded P-256 elliptic curve Diffie-Hellman public key

    #[serde(default)]
    p256dh_base64url: String,
    /// Base64url-encoded authentication secret

    #[serde(default)]
    auth_base64url: String,
}

impl RObject for DeviceTokenWebPush {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDDeviceToken for DeviceTokenWebPush {}

impl DeviceTokenWebPush {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DeviceTokenWebPushBuilder {
        let mut inner = DeviceTokenWebPush::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        DeviceTokenWebPushBuilder { inner }
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
pub struct DeviceTokenWebPushBuilder {
    inner: DeviceTokenWebPush,
}

#[deprecated]
pub type RTDDeviceTokenWebPushBuilder = DeviceTokenWebPushBuilder;

impl DeviceTokenWebPushBuilder {
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

impl AsRef<DeviceTokenWebPush> for DeviceTokenWebPushBuilder {
    fn as_ref(&self) -> &DeviceTokenWebPush {
        &self.inner
    }
}

/// A token for Windows Push Notification Services
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceTokenWindowsPush {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The access token that will be used to send notifications; may be empty to deregister a device

    #[serde(default)]
    access_token: String,
}

impl RObject for DeviceTokenWindowsPush {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDDeviceToken for DeviceTokenWindowsPush {}

impl DeviceTokenWindowsPush {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DeviceTokenWindowsPushBuilder {
        let mut inner = DeviceTokenWindowsPush::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        DeviceTokenWindowsPushBuilder { inner }
    }

    pub fn access_token(&self) -> &String {
        &self.access_token
    }
}

#[doc(hidden)]
pub struct DeviceTokenWindowsPushBuilder {
    inner: DeviceTokenWindowsPush,
}

#[deprecated]
pub type RTDDeviceTokenWindowsPushBuilder = DeviceTokenWindowsPushBuilder;

impl DeviceTokenWindowsPushBuilder {
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

impl AsRef<DeviceTokenWindowsPush> for DeviceTokenWindowsPushBuilder {
    fn as_ref(&self) -> &DeviceTokenWindowsPush {
        &self.inner
    }
}
