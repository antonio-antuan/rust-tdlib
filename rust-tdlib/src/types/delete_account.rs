use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Deletes the account of the current user, deleting all information associated with the user from the server. The phone number of the account can be used to create a new account. Can be called before authorization when the current authorization state is authorizationStateWaitPassword
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteAccount {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The reason why the account was deleted; optional
    reason: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DeleteAccount {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DeleteAccount {}

impl DeleteAccount {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeleteAccountBuilder {
        let mut inner = DeleteAccount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "deleteAccount".to_string();

        RTDDeleteAccountBuilder { inner }
    }

    pub fn reason(&self) -> &String {
        &self.reason
    }
}

#[doc(hidden)]
pub struct RTDDeleteAccountBuilder {
    inner: DeleteAccount,
}

impl RTDDeleteAccountBuilder {
    pub fn build(&self) -> DeleteAccount {
        self.inner.clone()
    }

    pub fn reason<T: AsRef<str>>(&mut self, reason: T) -> &mut Self {
        self.inner.reason = reason.as_ref().to_string();
        self
    }
}

impl AsRef<DeleteAccount> for DeleteAccount {
    fn as_ref(&self) -> &DeleteAccount {
        self
    }
}

impl AsRef<DeleteAccount> for RTDDeleteAccountBuilder {
    fn as_ref(&self) -> &DeleteAccount {
        &self.inner
    }
}
