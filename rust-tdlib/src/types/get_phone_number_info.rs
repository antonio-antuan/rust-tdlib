use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a phone number by its prefix. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPhoneNumberInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The phone number prefix
    phone_number_prefix: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetPhoneNumberInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetPhoneNumberInfo {}

impl GetPhoneNumberInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetPhoneNumberInfoBuilder {
        let mut inner = GetPhoneNumberInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getPhoneNumberInfo".to_string();

        RTDGetPhoneNumberInfoBuilder { inner }
    }

    pub fn phone_number_prefix(&self) -> &String {
        &self.phone_number_prefix
    }
}

#[doc(hidden)]
pub struct RTDGetPhoneNumberInfoBuilder {
    inner: GetPhoneNumberInfo,
}

impl RTDGetPhoneNumberInfoBuilder {
    pub fn build(&self) -> GetPhoneNumberInfo {
        self.inner.clone()
    }

    pub fn phone_number_prefix<T: AsRef<str>>(&mut self, phone_number_prefix: T) -> &mut Self {
        self.inner.phone_number_prefix = phone_number_prefix.as_ref().to_string();
        self
    }
}

impl AsRef<GetPhoneNumberInfo> for GetPhoneNumberInfo {
    fn as_ref(&self) -> &GetPhoneNumberInfo {
        self
    }
}

impl AsRef<GetPhoneNumberInfo> for RTDGetPhoneNumberInfoBuilder {
    fn as_ref(&self) -> &GetPhoneNumberInfo {
        &self.inner
    }
}
