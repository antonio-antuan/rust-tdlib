use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns already available Telegram Passport elements suitable for completing a Telegram Passport authorization form. Result can be received only once for each authorization form
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPassportAuthorizationFormAvailableElements {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Authorization form identifier
    autorization_form_id: i32,
    /// Password of the current user
    password: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetPassportAuthorizationFormAvailableElements {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetPassportAuthorizationFormAvailableElements {}

impl GetPassportAuthorizationFormAvailableElements {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetPassportAuthorizationFormAvailableElementsBuilder {
        let mut inner = GetPassportAuthorizationFormAvailableElements::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getPassportAuthorizationFormAvailableElements".to_string();

        RTDGetPassportAuthorizationFormAvailableElementsBuilder { inner }
    }

    pub fn autorization_form_id(&self) -> i32 {
        self.autorization_form_id
    }

    pub fn password(&self) -> &String {
        &self.password
    }
}

#[doc(hidden)]
pub struct RTDGetPassportAuthorizationFormAvailableElementsBuilder {
    inner: GetPassportAuthorizationFormAvailableElements,
}

impl RTDGetPassportAuthorizationFormAvailableElementsBuilder {
    pub fn build(&self) -> GetPassportAuthorizationFormAvailableElements {
        self.inner.clone()
    }

    pub fn autorization_form_id(&mut self, autorization_form_id: i32) -> &mut Self {
        self.inner.autorization_form_id = autorization_form_id;
        self
    }

    pub fn password<T: AsRef<str>>(&mut self, password: T) -> &mut Self {
        self.inner.password = password.as_ref().to_string();
        self
    }
}

impl AsRef<GetPassportAuthorizationFormAvailableElements>
    for GetPassportAuthorizationFormAvailableElements
{
    fn as_ref(&self) -> &GetPassportAuthorizationFormAvailableElements {
        self
    }
}

impl AsRef<GetPassportAuthorizationFormAvailableElements>
    for RTDGetPassportAuthorizationFormAvailableElementsBuilder
{
    fn as_ref(&self) -> &GetPassportAuthorizationFormAvailableElements {
        &self.inner
    }
}
