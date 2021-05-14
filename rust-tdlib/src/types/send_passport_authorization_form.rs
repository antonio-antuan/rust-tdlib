use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Sends a Telegram Passport authorization form, effectively sharing data with the service. This method must be called after getPassportAuthorizationFormAvailableElements if some previously available elements are going to be reused
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendPassportAuthorizationForm {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Authorization form identifier
    autorization_form_id: i32,
    /// Types of Telegram Passport elements chosen by user to complete the authorization form
    types: Vec<PassportElementType>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SendPassportAuthorizationForm {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SendPassportAuthorizationForm {}

impl SendPassportAuthorizationForm {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSendPassportAuthorizationFormBuilder {
        let mut inner = SendPassportAuthorizationForm::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "sendPassportAuthorizationForm".to_string();

        RTDSendPassportAuthorizationFormBuilder { inner }
    }

    pub fn autorization_form_id(&self) -> i32 {
        self.autorization_form_id
    }

    pub fn types(&self) -> &Vec<PassportElementType> {
        &self.types
    }
}

#[doc(hidden)]
pub struct RTDSendPassportAuthorizationFormBuilder {
    inner: SendPassportAuthorizationForm,
}

impl RTDSendPassportAuthorizationFormBuilder {
    pub fn build(&self) -> SendPassportAuthorizationForm {
        self.inner.clone()
    }

    pub fn autorization_form_id(&mut self, autorization_form_id: i32) -> &mut Self {
        self.inner.autorization_form_id = autorization_form_id;
        self
    }

    pub fn types(&mut self, types: Vec<PassportElementType>) -> &mut Self {
        self.inner.types = types;
        self
    }
}

impl AsRef<SendPassportAuthorizationForm> for SendPassportAuthorizationForm {
    fn as_ref(&self) -> &SendPassportAuthorizationForm {
        self
    }
}

impl AsRef<SendPassportAuthorizationForm> for RTDSendPassportAuthorizationFormBuilder {
    fn as_ref(&self) -> &SendPassportAuthorizationForm {
        &self.inner
    }
}
