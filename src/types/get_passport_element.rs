use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns one of the available Telegram Passport elements
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPassportElement {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Telegram Passport element type

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "PassportElementType::_is_default")]
    type_: PassportElementType,
    /// Password of the current user
    password: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetPassportElement {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElement for GetPassportElement {}

impl RFunction for GetPassportElement {}

impl GetPassportElement {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetPassportElementBuilder {
        let mut inner = GetPassportElement::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getPassportElement".to_string();

        RTDGetPassportElementBuilder { inner }
    }

    pub fn type_(&self) -> &PassportElementType {
        &self.type_
    }

    pub fn password(&self) -> &String {
        &self.password
    }
}

#[doc(hidden)]
pub struct RTDGetPassportElementBuilder {
    inner: GetPassportElement,
}

impl RTDGetPassportElementBuilder {
    pub fn build(&self) -> GetPassportElement {
        self.inner.clone()
    }

    pub fn type_<T: AsRef<PassportElementType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }

    pub fn password<T: AsRef<str>>(&mut self, password: T) -> &mut Self {
        self.inner.password = password.as_ref().to_string();
        self
    }
}

impl AsRef<GetPassportElement> for GetPassportElement {
    fn as_ref(&self) -> &GetPassportElement {
        self
    }
}

impl AsRef<GetPassportElement> for RTDGetPassportElementBuilder {
    fn as_ref(&self) -> &GetPassportElement {
        &self.inner
    }
}
