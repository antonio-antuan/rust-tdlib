use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns all available Telegram Passport elements
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAllPassportElements {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Password of the current user
    password: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetAllPassportElements {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetAllPassportElements {}

impl GetAllPassportElements {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetAllPassportElementsBuilder {
        let mut inner = GetAllPassportElements::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getAllPassportElements".to_string();

        RTDGetAllPassportElementsBuilder { inner }
    }

    pub fn password(&self) -> &String {
        &self.password
    }
}

#[doc(hidden)]
pub struct RTDGetAllPassportElementsBuilder {
    inner: GetAllPassportElements,
}

impl RTDGetAllPassportElementsBuilder {
    pub fn build(&self) -> GetAllPassportElements {
        self.inner.clone()
    }

    pub fn password<T: AsRef<str>>(&mut self, password: T) -> &mut Self {
        self.inner.password = password.as_ref().to_string();
        self
    }
}

impl AsRef<GetAllPassportElements> for GetAllPassportElements {
    fn as_ref(&self) -> &GetAllPassportElements {
        self
    }
}

impl AsRef<GetAllPassportElements> for RTDGetAllPassportElementsBuilder {
    fn as_ref(&self) -> &GetAllPassportElements {
        &self.inner
    }
}
