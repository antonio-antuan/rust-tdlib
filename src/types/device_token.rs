use crate::errors::*;
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
    #[serde(rename(
        serialize = "deviceTokenApplePush",
        deserialize = "deviceTokenApplePush"
    ))]
    ApplePush(DeviceTokenApplePush),
    /// A token for Apple Push Notification service VoIP notifications
    #[serde(rename(
        serialize = "deviceTokenApplePushVoIP",
        deserialize = "deviceTokenApplePushVoIP"
    ))]
    ApplePushVoIP(DeviceTokenApplePushVoIP),
    /// A token for BlackBerry Push Service
    #[serde(rename(
        serialize = "deviceTokenBlackBerryPush",
        deserialize = "deviceTokenBlackBerryPush"
    ))]
    BlackBerryPush(DeviceTokenBlackBerryPush),
    /// A token for Firebase Cloud Messaging
    #[serde(rename(
        serialize = "deviceTokenFirebaseCloudMessaging",
        deserialize = "deviceTokenFirebaseCloudMessaging"
    ))]
    FirebaseCloudMessaging(DeviceTokenFirebaseCloudMessaging),
    /// A token for Microsoft Push Notification Service
    #[serde(rename(
        serialize = "deviceTokenMicrosoftPush",
        deserialize = "deviceTokenMicrosoftPush"
    ))]
    MicrosoftPush(DeviceTokenMicrosoftPush),
    /// A token for Microsoft Push Notification Service VoIP channel
    #[serde(rename(
        serialize = "deviceTokenMicrosoftPushVoIP",
        deserialize = "deviceTokenMicrosoftPushVoIP"
    ))]
    MicrosoftPushVoIP(DeviceTokenMicrosoftPushVoIP),
    /// A token for Simple Push API for Firefox OS
    #[serde(rename(
        serialize = "deviceTokenSimplePush",
        deserialize = "deviceTokenSimplePush"
    ))]
    SimplePush(DeviceTokenSimplePush),
    /// A token for Tizen Push Service
    #[serde(rename(
        serialize = "deviceTokenTizenPush",
        deserialize = "deviceTokenTizenPush"
    ))]
    TizenPush(DeviceTokenTizenPush),
    /// A token for Ubuntu Push Client service
    #[serde(rename(
        serialize = "deviceTokenUbuntuPush",
        deserialize = "deviceTokenUbuntuPush"
    ))]
    UbuntuPush(DeviceTokenUbuntuPush),
    /// A token for web Push API
    #[serde(rename(serialize = "deviceTokenWebPush", deserialize = "deviceTokenWebPush"))]
    WebPush(DeviceTokenWebPush),
    /// A token for Windows Push Notification Services
    #[serde(rename(
        serialize = "deviceTokenWindowsPush",
        deserialize = "deviceTokenWindowsPush"
    ))]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
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
    /// Device token; may be empty to de-register a device
    device_token: String,
    /// True, if App Sandbox is enabled
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeviceTokenApplePushBuilder {
        let mut inner = DeviceTokenApplePush::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeviceTokenApplePushVoIPBuilder {
        let mut inner = DeviceTokenApplePushVoIP::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Token; may be empty to de-register a device
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeviceTokenBlackBerryPushBuilder {
        let mut inner = DeviceTokenBlackBerryPush::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeviceTokenFirebaseCloudMessagingBuilder {
        let mut inner = DeviceTokenFirebaseCloudMessaging::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Push notification channel URI; may be empty to de-register a device
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeviceTokenMicrosoftPushBuilder {
        let mut inner = DeviceTokenMicrosoftPush::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Push notification channel URI; may be empty to de-register a device
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeviceTokenMicrosoftPushVoIPBuilder {
        let mut inner = DeviceTokenMicrosoftPushVoIP::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Absolute URL exposed by the push service where the application server can send push messages; may be empty to de-register a device
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeviceTokenSimplePushBuilder {
        let mut inner = DeviceTokenSimplePush::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Push service registration identifier; may be empty to de-register a device
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeviceTokenTizenPushBuilder {
        let mut inner = DeviceTokenTizenPush::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Token; may be empty to de-register a device
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeviceTokenUbuntuPushBuilder {
        let mut inner = DeviceTokenUbuntuPush::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeviceTokenWebPushBuilder {
        let mut inner = DeviceTokenWebPush::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The access token that will be used to send notifications; may be empty to de-register a device
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeviceTokenWindowsPushBuilder {
        let mut inner = DeviceTokenWindowsPush::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
