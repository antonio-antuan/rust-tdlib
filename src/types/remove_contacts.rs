use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Removes users from the contact list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveContacts {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifiers of users to be deleted
    user_ids: Vec<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for RemoveContacts {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RemoveContacts {}

impl RemoveContacts {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRemoveContactsBuilder {
        let mut inner = RemoveContacts::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "removeContacts".to_string();

        RTDRemoveContactsBuilder { inner }
    }

    pub fn user_ids(&self) -> &Vec<i32> {
        &self.user_ids
    }
}

#[doc(hidden)]
pub struct RTDRemoveContactsBuilder {
    inner: RemoveContacts,
}

impl RTDRemoveContactsBuilder {
    pub fn build(&self) -> RemoveContacts {
        self.inner.clone()
    }

    pub fn user_ids(&mut self, user_ids: Vec<i32>) -> &mut Self {
        self.inner.user_ids = user_ids;
        self
    }
}

impl AsRef<RemoveContacts> for RemoveContacts {
    fn as_ref(&self) -> &RemoveContacts {
        self
    }
}

impl AsRef<RemoveContacts> for RTDRemoveContactsBuilder {
    fn as_ref(&self) -> &RemoveContacts {
        &self.inner
    }
}
