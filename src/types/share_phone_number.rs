use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Shares the phone number of the current user with a mutual contact. Supposed to be called when the user clicks on chatActionBarSharePhoneNumber
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SharePhoneNumber {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the user with whom to share the phone number. The user must be a mutual contact
    user_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SharePhoneNumber {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SharePhoneNumber {}

impl SharePhoneNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSharePhoneNumberBuilder {
        let mut inner = SharePhoneNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "sharePhoneNumber".to_string();

        RTDSharePhoneNumberBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}

#[doc(hidden)]
pub struct RTDSharePhoneNumberBuilder {
    inner: SharePhoneNumber,
}

impl RTDSharePhoneNumberBuilder {
    pub fn build(&self) -> SharePhoneNumber {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }
}

impl AsRef<SharePhoneNumber> for SharePhoneNumber {
    fn as_ref(&self) -> &SharePhoneNumber {
        self
    }
}

impl AsRef<SharePhoneNumber> for RTDSharePhoneNumberBuilder {
    fn as_ref(&self) -> &SharePhoneNumber {
        &self.inner
    }
}
