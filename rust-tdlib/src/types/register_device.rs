use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Registers the currently used device for receiving push notifications. Returns a globally unique identifier of the push notification subscription
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RegisterDevice {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Device token

    #[serde(skip_serializing_if = "DeviceToken::_is_default")]
    device_token: DeviceToken,
    /// List of user identifiers of other users currently using the application
    other_user_ids: Vec<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for RegisterDevice {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RegisterDevice {}

impl RegisterDevice {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRegisterDeviceBuilder {
        let mut inner = RegisterDevice::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "registerDevice".to_string();

        RTDRegisterDeviceBuilder { inner }
    }

    pub fn device_token(&self) -> &DeviceToken {
        &self.device_token
    }

    pub fn other_user_ids(&self) -> &Vec<i32> {
        &self.other_user_ids
    }
}

#[doc(hidden)]
pub struct RTDRegisterDeviceBuilder {
    inner: RegisterDevice,
}

impl RTDRegisterDeviceBuilder {
    pub fn build(&self) -> RegisterDevice {
        self.inner.clone()
    }

    pub fn device_token<T: AsRef<DeviceToken>>(&mut self, device_token: T) -> &mut Self {
        self.inner.device_token = device_token.as_ref().clone();
        self
    }

    pub fn other_user_ids(&mut self, other_user_ids: Vec<i32>) -> &mut Self {
        self.inner.other_user_ids = other_user_ids;
        self
    }
}

impl AsRef<RegisterDevice> for RegisterDevice {
    fn as_ref(&self) -> &RegisterDevice {
        self
    }
}

impl AsRef<RegisterDevice> for RTDRegisterDeviceBuilder {
    fn as_ref(&self) -> &RegisterDevice {
        &self.inner
    }
}
