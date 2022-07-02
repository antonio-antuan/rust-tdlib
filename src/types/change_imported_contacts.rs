use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes imported contacts using the list of contacts saved on the device. Imports newly added contacts and, if at least the file database is enabled, deletes recently deleted contacts. Query result depends on the result of the previous query, so only one query is possible at the same time
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangeImportedContacts {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The new list of contacts, contact's vCard are ignored and are not imported

    #[serde(default)]
    contacts: Vec<Contact>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ChangeImportedContacts {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ChangeImportedContacts {}

impl ChangeImportedContacts {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChangeImportedContactsBuilder {
        let mut inner = ChangeImportedContacts::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "changeImportedContacts".to_string();

        ChangeImportedContactsBuilder { inner }
    }

    pub fn contacts(&self) -> &Vec<Contact> {
        &self.contacts
    }
}

#[doc(hidden)]
pub struct ChangeImportedContactsBuilder {
    inner: ChangeImportedContacts,
}

#[deprecated]
pub type RTDChangeImportedContactsBuilder = ChangeImportedContactsBuilder;

impl ChangeImportedContactsBuilder {
    pub fn build(&self) -> ChangeImportedContacts {
        self.inner.clone()
    }

    pub fn contacts(&mut self, contacts: Vec<Contact>) -> &mut Self {
        self.inner.contacts = contacts;
        self
    }
}

impl AsRef<ChangeImportedContacts> for ChangeImportedContacts {
    fn as_ref(&self) -> &ChangeImportedContacts {
        self
    }
}

impl AsRef<ChangeImportedContacts> for ChangeImportedContactsBuilder {
    fn as_ref(&self) -> &ChangeImportedContacts {
        &self.inner
    }
}
