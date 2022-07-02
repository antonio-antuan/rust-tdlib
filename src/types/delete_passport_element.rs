use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Deletes a Telegram Passport element
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeletePassportElement {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Element type

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "PassportElementType::_is_default")]
    type_: PassportElementType,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DeletePassportElement {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DeletePassportElement {}

impl DeletePassportElement {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DeletePassportElementBuilder {
        let mut inner = DeletePassportElement::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "deletePassportElement".to_string();

        DeletePassportElementBuilder { inner }
    }

    pub fn type_(&self) -> &PassportElementType {
        &self.type_
    }
}

#[doc(hidden)]
pub struct DeletePassportElementBuilder {
    inner: DeletePassportElement,
}

#[deprecated]
pub type RTDDeletePassportElementBuilder = DeletePassportElementBuilder;

impl DeletePassportElementBuilder {
    pub fn build(&self) -> DeletePassportElement {
        self.inner.clone()
    }

    pub fn type_<T: AsRef<PassportElementType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }
}

impl AsRef<DeletePassportElement> for DeletePassportElement {
    fn as_ref(&self) -> &DeletePassportElement {
        self
    }
}

impl AsRef<DeletePassportElement> for DeletePassportElementBuilder {
    fn as_ref(&self) -> &DeletePassportElement {
        &self.inner
    }
}
