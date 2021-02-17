use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Accepts Telegram terms of services
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AcceptTermsOfService {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Terms of service identifier
    terms_of_service_id: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AcceptTermsOfService {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AcceptTermsOfService {}

impl AcceptTermsOfService {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDAcceptTermsOfServiceBuilder {
        let mut inner = AcceptTermsOfService::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "acceptTermsOfService".to_string();

        RTDAcceptTermsOfServiceBuilder { inner }
    }

    pub fn terms_of_service_id(&self) -> &String {
        &self.terms_of_service_id
    }
}

#[doc(hidden)]
pub struct RTDAcceptTermsOfServiceBuilder {
    inner: AcceptTermsOfService,
}

impl RTDAcceptTermsOfServiceBuilder {
    pub fn build(&self) -> AcceptTermsOfService {
        self.inner.clone()
    }

    pub fn terms_of_service_id<T: AsRef<str>>(&mut self, terms_of_service_id: T) -> &mut Self {
        self.inner.terms_of_service_id = terms_of_service_id.as_ref().to_string();
        self
    }
}

impl AsRef<AcceptTermsOfService> for AcceptTermsOfService {
    fn as_ref(&self) -> &AcceptTermsOfService {
        self
    }
}

impl AsRef<AcceptTermsOfService> for RTDAcceptTermsOfServiceBuilder {
    fn as_ref(&self) -> &AcceptTermsOfService {
        &self.inner
    }
}
