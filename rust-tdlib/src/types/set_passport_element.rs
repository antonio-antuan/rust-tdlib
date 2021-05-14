use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Adds an element to the user's Telegram Passport. May return an error with a message "PHONE_VERIFICATION_NEEDED" or "EMAIL_VERIFICATION_NEEDED" if the chosen phone number or the chosen email address must be verified first
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetPassportElement {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Input Telegram Passport element

    #[serde(skip_serializing_if = "InputPassportElement::_is_default")]
    element: InputPassportElement,
    /// Password of the current user
    password: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetPassportElement {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElement for SetPassportElement {}

impl RFunction for SetPassportElement {}

impl SetPassportElement {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetPassportElementBuilder {
        let mut inner = SetPassportElement::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setPassportElement".to_string();

        RTDSetPassportElementBuilder { inner }
    }

    pub fn element(&self) -> &InputPassportElement {
        &self.element
    }

    pub fn password(&self) -> &String {
        &self.password
    }
}

#[doc(hidden)]
pub struct RTDSetPassportElementBuilder {
    inner: SetPassportElement,
}

impl RTDSetPassportElementBuilder {
    pub fn build(&self) -> SetPassportElement {
        self.inner.clone()
    }

    pub fn element<T: AsRef<InputPassportElement>>(&mut self, element: T) -> &mut Self {
        self.inner.element = element.as_ref().clone();
        self
    }

    pub fn password<T: AsRef<str>>(&mut self, password: T) -> &mut Self {
        self.inner.password = password.as_ref().to_string();
        self
    }
}

impl AsRef<SetPassportElement> for SetPassportElement {
    fn as_ref(&self) -> &SetPassportElement {
        self
    }
}

impl AsRef<SetPassportElement> for RTDSetPassportElementBuilder {
    fn as_ref(&self) -> &SetPassportElement {
        &self.inner
    }
}
