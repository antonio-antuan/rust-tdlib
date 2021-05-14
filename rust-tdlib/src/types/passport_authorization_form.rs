use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a Telegram Passport authorization form that was requested
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportAuthorizationForm {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique identifier of the authorization form
    id: i32,
    /// Information about the Telegram Passport elements that must be provided to complete the form
    required_elements: Vec<PassportRequiredElement>,
    /// URL for the privacy policy of the service; may be empty
    privacy_policy_url: String,
}

impl RObject for PassportAuthorizationForm {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PassportAuthorizationForm {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportAuthorizationFormBuilder {
        let mut inner = PassportAuthorizationForm::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportAuthorizationFormBuilder { inner }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn required_elements(&self) -> &Vec<PassportRequiredElement> {
        &self.required_elements
    }

    pub fn privacy_policy_url(&self) -> &String {
        &self.privacy_policy_url
    }
}

#[doc(hidden)]
pub struct RTDPassportAuthorizationFormBuilder {
    inner: PassportAuthorizationForm,
}

impl RTDPassportAuthorizationFormBuilder {
    pub fn build(&self) -> PassportAuthorizationForm {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i32) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn required_elements(
        &mut self,
        required_elements: Vec<PassportRequiredElement>,
    ) -> &mut Self {
        self.inner.required_elements = required_elements;
        self
    }

    pub fn privacy_policy_url<T: AsRef<str>>(&mut self, privacy_policy_url: T) -> &mut Self {
        self.inner.privacy_policy_url = privacy_policy_url.as_ref().to_string();
        self
    }
}

impl AsRef<PassportAuthorizationForm> for PassportAuthorizationForm {
    fn as_ref(&self) -> &PassportAuthorizationForm {
        self
    }
}

impl AsRef<PassportAuthorizationForm> for RTDPassportAuthorizationFormBuilder {
    fn as_ref(&self) -> &PassportAuthorizationForm {
        &self.inner
    }
}
