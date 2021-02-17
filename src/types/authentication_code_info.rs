use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Information about the authentication code that was sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthenticationCodeInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A phone number that is being authenticated
    phone_number: String,
    /// Describes the way the code was sent to the user

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "AuthenticationCodeType::_is_default")]
    type_: AuthenticationCodeType,
    /// Describes the way the next code will be sent to the user; may be null
    next_type: Option<AuthenticationCodeType>,
    /// Timeout before the code should be re-sent, in seconds
    timeout: i32,
}

impl RObject for AuthenticationCodeInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl AuthenticationCodeInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDAuthenticationCodeInfoBuilder {
        let mut inner = AuthenticationCodeInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDAuthenticationCodeInfoBuilder { inner }
    }

    pub fn phone_number(&self) -> &String {
        &self.phone_number
    }

    pub fn type_(&self) -> &AuthenticationCodeType {
        &self.type_
    }

    pub fn next_type(&self) -> &Option<AuthenticationCodeType> {
        &self.next_type
    }

    pub fn timeout(&self) -> i32 {
        self.timeout
    }
}

#[doc(hidden)]
pub struct RTDAuthenticationCodeInfoBuilder {
    inner: AuthenticationCodeInfo,
}

impl RTDAuthenticationCodeInfoBuilder {
    pub fn build(&self) -> AuthenticationCodeInfo {
        self.inner.clone()
    }

    pub fn phone_number<T: AsRef<str>>(&mut self, phone_number: T) -> &mut Self {
        self.inner.phone_number = phone_number.as_ref().to_string();
        self
    }

    pub fn type_<T: AsRef<AuthenticationCodeType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }

    pub fn next_type<T: AsRef<AuthenticationCodeType>>(&mut self, next_type: T) -> &mut Self {
        self.inner.next_type = Some(next_type.as_ref().clone());
        self
    }

    pub fn timeout(&mut self, timeout: i32) -> &mut Self {
        self.inner.timeout = timeout;
        self
    }
}

impl AsRef<AuthenticationCodeInfo> for AuthenticationCodeInfo {
    fn as_ref(&self) -> &AuthenticationCodeInfo {
        self
    }
}

impl AsRef<AuthenticationCodeInfo> for RTDAuthenticationCodeInfoBuilder {
    fn as_ref(&self) -> &AuthenticationCodeInfo {
        &self.inner
    }
}
