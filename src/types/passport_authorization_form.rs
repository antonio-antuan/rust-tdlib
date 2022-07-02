use crate::errors::Result;
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

    #[serde(default)]
    id: i32,
    /// Telegram Passport elements that must be provided to complete the form

    #[serde(default)]
    required_elements: Vec<PassportRequiredElement>,
    /// URL for the privacy policy of the service; may be empty

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportAuthorizationFormBuilder {
        let mut inner = PassportAuthorizationForm::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportAuthorizationFormBuilder { inner }
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
pub struct PassportAuthorizationFormBuilder {
    inner: PassportAuthorizationForm,
}

#[deprecated]
pub type RTDPassportAuthorizationFormBuilder = PassportAuthorizationFormBuilder;

impl PassportAuthorizationFormBuilder {
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

impl AsRef<PassportAuthorizationForm> for PassportAuthorizationFormBuilder {
    fn as_ref(&self) -> &PassportAuthorizationForm {
        &self.inner
    }
}
