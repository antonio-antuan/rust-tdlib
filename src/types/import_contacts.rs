use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Adds new contacts or edits existing contacts by their phone numbers; contacts' user identifiers are ignored
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImportContacts {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The list of contacts to import or edit; contacts' vCard are ignored and are not imported

    #[serde(default)]
    contacts: Vec<Contact>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ImportContacts {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ImportContacts {}

impl ImportContacts {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ImportContactsBuilder {
        let mut inner = ImportContacts::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "importContacts".to_string();

        ImportContactsBuilder { inner }
    }

    pub fn contacts(&self) -> &Vec<Contact> {
        &self.contacts
    }
}

#[doc(hidden)]
pub struct ImportContactsBuilder {
    inner: ImportContacts,
}

#[deprecated]
pub type RTDImportContactsBuilder = ImportContactsBuilder;

impl ImportContactsBuilder {
    pub fn build(&self) -> ImportContacts {
        self.inner.clone()
    }

    pub fn contacts(&mut self, contacts: Vec<Contact>) -> &mut Self {
        self.inner.contacts = contacts;
        self
    }
}

impl AsRef<ImportContacts> for ImportContacts {
    fn as_ref(&self) -> &ImportContacts {
        self
    }
}

impl AsRef<ImportContacts> for ImportContactsBuilder {
    fn as_ref(&self) -> &ImportContacts {
        &self.inner
    }
}
