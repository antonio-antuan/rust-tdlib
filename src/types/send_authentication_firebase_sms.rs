use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sends Firebase Authentication SMS to the phone number of the user. Works only when the current authorization state is authorizationStateWaitCode and the server returned code of the type authenticationCodeTypeFirebaseAndroid or authenticationCodeTypeFirebaseIos
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendAuthenticationFirebaseSms {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// SafetyNet Attestation API token for the Android application, or secret from push notification for the iOS application

    #[serde(default)]
    token: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SendAuthenticationFirebaseSms {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SendAuthenticationFirebaseSms {}

impl SendAuthenticationFirebaseSms {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SendAuthenticationFirebaseSmsBuilder {
        let mut inner = SendAuthenticationFirebaseSms::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "sendAuthenticationFirebaseSms".to_string();

        SendAuthenticationFirebaseSmsBuilder { inner }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

#[doc(hidden)]
pub struct SendAuthenticationFirebaseSmsBuilder {
    inner: SendAuthenticationFirebaseSms,
}

#[deprecated]
pub type RTDSendAuthenticationFirebaseSmsBuilder = SendAuthenticationFirebaseSmsBuilder;

impl SendAuthenticationFirebaseSmsBuilder {
    pub fn build(&self) -> SendAuthenticationFirebaseSms {
        self.inner.clone()
    }

    pub fn token<T: AsRef<str>>(&mut self, token: T) -> &mut Self {
        self.inner.token = token.as_ref().to_string();
        self
    }
}

impl AsRef<SendAuthenticationFirebaseSms> for SendAuthenticationFirebaseSms {
    fn as_ref(&self) -> &SendAuthenticationFirebaseSms {
        self
    }
}

impl AsRef<SendAuthenticationFirebaseSms> for SendAuthenticationFirebaseSmsBuilder {
    fn as_ref(&self) -> &SendAuthenticationFirebaseSms {
        &self.inner
    }
}
