use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a description of the required Telegram Passport element that was requested by a service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportRequiredElement {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of Telegram Passport elements any of which is enough to provide

    #[serde(default)]
    suitable_elements: Vec<PassportSuitableElement>,
}

impl RObject for PassportRequiredElement {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PassportRequiredElement {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportRequiredElementBuilder {
        let mut inner = PassportRequiredElement::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportRequiredElementBuilder { inner }
    }

    pub fn suitable_elements(&self) -> &Vec<PassportSuitableElement> {
        &self.suitable_elements
    }
}

#[doc(hidden)]
pub struct PassportRequiredElementBuilder {
    inner: PassportRequiredElement,
}

#[deprecated]
pub type RTDPassportRequiredElementBuilder = PassportRequiredElementBuilder;

impl PassportRequiredElementBuilder {
    pub fn build(&self) -> PassportRequiredElement {
        self.inner.clone()
    }

    pub fn suitable_elements(
        &mut self,
        suitable_elements: Vec<PassportSuitableElement>,
    ) -> &mut Self {
        self.inner.suitable_elements = suitable_elements;
        self
    }
}

impl AsRef<PassportRequiredElement> for PassportRequiredElement {
    fn as_ref(&self) -> &PassportRequiredElement {
        self
    }
}

impl AsRef<PassportRequiredElement> for PassportRequiredElementBuilder {
    fn as_ref(&self) -> &PassportRequiredElement {
        &self.inner
    }
}
