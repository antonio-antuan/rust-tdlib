use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns a Telegram Passport authorization form for sharing data with a service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPassportAuthorizationForm {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifier of the service's bot
    bot_user_id: i32,
    /// Telegram Passport element types requested by the service
    scope: String,
    /// Service's public_key
    public_key: String,
    /// Authorization form nonce provided by the service
    nonce: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetPassportAuthorizationForm {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetPassportAuthorizationForm {}

impl GetPassportAuthorizationForm {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetPassportAuthorizationFormBuilder {
        let mut inner = GetPassportAuthorizationForm::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getPassportAuthorizationForm".to_string();

        RTDGetPassportAuthorizationFormBuilder { inner }
    }

    pub fn bot_user_id(&self) -> i32 {
        self.bot_user_id
    }

    pub fn scope(&self) -> &String {
        &self.scope
    }

    pub fn public_key(&self) -> &String {
        &self.public_key
    }

    pub fn nonce(&self) -> &String {
        &self.nonce
    }
}

#[doc(hidden)]
pub struct RTDGetPassportAuthorizationFormBuilder {
    inner: GetPassportAuthorizationForm,
}

impl RTDGetPassportAuthorizationFormBuilder {
    pub fn build(&self) -> GetPassportAuthorizationForm {
        self.inner.clone()
    }

    pub fn bot_user_id(&mut self, bot_user_id: i32) -> &mut Self {
        self.inner.bot_user_id = bot_user_id;
        self
    }

    pub fn scope<T: AsRef<str>>(&mut self, scope: T) -> &mut Self {
        self.inner.scope = scope.as_ref().to_string();
        self
    }

    pub fn public_key<T: AsRef<str>>(&mut self, public_key: T) -> &mut Self {
        self.inner.public_key = public_key.as_ref().to_string();
        self
    }

    pub fn nonce<T: AsRef<str>>(&mut self, nonce: T) -> &mut Self {
        self.inner.nonce = nonce.as_ref().to_string();
        self
    }
}

impl AsRef<GetPassportAuthorizationForm> for GetPassportAuthorizationForm {
    fn as_ref(&self) -> &GetPassportAuthorizationForm {
        self
    }
}

impl AsRef<GetPassportAuthorizationForm> for RTDGetPassportAuthorizationFormBuilder {
    fn as_ref(&self) -> &GetPassportAuthorizationForm {
        &self.inner
    }
}
