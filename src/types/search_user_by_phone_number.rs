use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Searches a user by their phone number. Returns a 404 error if the user can't be found
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchUserByPhoneNumber {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Phone number to search for
    phone_number: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SearchUserByPhoneNumber {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SearchUserByPhoneNumber {}

impl SearchUserByPhoneNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchUserByPhoneNumberBuilder {
        let mut inner = SearchUserByPhoneNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchUserByPhoneNumber".to_string();

        RTDSearchUserByPhoneNumberBuilder { inner }
    }

    pub fn phone_number(&self) -> &String {
        &self.phone_number
    }
}

#[doc(hidden)]
pub struct RTDSearchUserByPhoneNumberBuilder {
    inner: SearchUserByPhoneNumber,
}

impl RTDSearchUserByPhoneNumberBuilder {
    pub fn build(&self) -> SearchUserByPhoneNumber {
        self.inner.clone()
    }

    pub fn phone_number<T: AsRef<str>>(&mut self, phone_number: T) -> &mut Self {
        self.inner.phone_number = phone_number.as_ref().to_string();
        self
    }
}

impl AsRef<SearchUserByPhoneNumber> for SearchUserByPhoneNumber {
    fn as_ref(&self) -> &SearchUserByPhoneNumber {
        self
    }
}

impl AsRef<SearchUserByPhoneNumber> for RTDSearchUserByPhoneNumberBuilder {
    fn as_ref(&self) -> &SearchUserByPhoneNumber {
        &self.inner
    }
}
