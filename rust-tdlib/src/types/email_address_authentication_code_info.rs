use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Information about the email address authentication code that was sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmailAddressAuthenticationCodeInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Pattern of the email address to which an authentication code was sent
    email_address_pattern: String,
    /// Length of the code; 0 if unknown
    length: i32,
}

impl RObject for EmailAddressAuthenticationCodeInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl EmailAddressAuthenticationCodeInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDEmailAddressAuthenticationCodeInfoBuilder {
        let mut inner = EmailAddressAuthenticationCodeInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDEmailAddressAuthenticationCodeInfoBuilder { inner }
    }

    pub fn email_address_pattern(&self) -> &String {
        &self.email_address_pattern
    }

    pub fn length(&self) -> i32 {
        self.length
    }
}

#[doc(hidden)]
pub struct RTDEmailAddressAuthenticationCodeInfoBuilder {
    inner: EmailAddressAuthenticationCodeInfo,
}

impl RTDEmailAddressAuthenticationCodeInfoBuilder {
    pub fn build(&self) -> EmailAddressAuthenticationCodeInfo {
        self.inner.clone()
    }

    pub fn email_address_pattern<T: AsRef<str>>(&mut self, email_address_pattern: T) -> &mut Self {
        self.inner.email_address_pattern = email_address_pattern.as_ref().to_string();
        self
    }

    pub fn length(&mut self, length: i32) -> &mut Self {
        self.inner.length = length;
        self
    }
}

impl AsRef<EmailAddressAuthenticationCodeInfo> for EmailAddressAuthenticationCodeInfo {
    fn as_ref(&self) -> &EmailAddressAuthenticationCodeInfo {
        self
    }
}

impl AsRef<EmailAddressAuthenticationCodeInfo> for RTDEmailAddressAuthenticationCodeInfoBuilder {
    fn as_ref(&self) -> &EmailAddressAuthenticationCodeInfo {
        &self.inner
    }
}
