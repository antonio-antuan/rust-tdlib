use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a Telegram Passport element that was requested by a service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportSuitableElement {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Type of the element

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "PassportElementType::_is_default")]
    type_: PassportElementType,
    /// True, if a selfie is required with the identity document

    #[serde(default)]
    is_selfie_required: bool,
    /// True, if a certified English translation is required with the document

    #[serde(default)]
    is_translation_required: bool,
    /// True, if personal details must include the user's name in the language of their country of residence

    #[serde(default)]
    is_native_name_required: bool,
}

impl RObject for PassportSuitableElement {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PassportSuitableElement {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportSuitableElementBuilder {
        let mut inner = PassportSuitableElement::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportSuitableElementBuilder { inner }
    }

    pub fn type_(&self) -> &PassportElementType {
        &self.type_
    }

    pub fn is_selfie_required(&self) -> bool {
        self.is_selfie_required
    }

    pub fn is_translation_required(&self) -> bool {
        self.is_translation_required
    }

    pub fn is_native_name_required(&self) -> bool {
        self.is_native_name_required
    }
}

#[doc(hidden)]
pub struct PassportSuitableElementBuilder {
    inner: PassportSuitableElement,
}

#[deprecated]
pub type RTDPassportSuitableElementBuilder = PassportSuitableElementBuilder;

impl PassportSuitableElementBuilder {
    pub fn build(&self) -> PassportSuitableElement {
        self.inner.clone()
    }

    pub fn type_<T: AsRef<PassportElementType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }

    pub fn is_selfie_required(&mut self, is_selfie_required: bool) -> &mut Self {
        self.inner.is_selfie_required = is_selfie_required;
        self
    }

    pub fn is_translation_required(&mut self, is_translation_required: bool) -> &mut Self {
        self.inner.is_translation_required = is_translation_required;
        self
    }

    pub fn is_native_name_required(&mut self, is_native_name_required: bool) -> &mut Self {
        self.inner.is_native_name_required = is_native_name_required;
        self
    }
}

impl AsRef<PassportSuitableElement> for PassportSuitableElement {
    fn as_ref(&self) -> &PassportSuitableElement {
        self
    }
}

impl AsRef<PassportSuitableElement> for PassportSuitableElementBuilder {
    fn as_ref(&self) -> &PassportSuitableElement {
        &self.inner
    }
}
