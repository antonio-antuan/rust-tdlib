use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents the result of an ImportContacts request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImportedContacts {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifiers of the imported contacts in the same order as they were specified in the request; 0 if the contact is not yet a registered user
    user_ids: Vec<i32>,
    /// The number of users that imported the corresponding contact; 0 for already registered users or if unavailable
    importer_count: Vec<i32>,
}

impl RObject for ImportedContacts {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ImportedContacts {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDImportedContactsBuilder {
        let mut inner = ImportedContacts::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDImportedContactsBuilder { inner }
    }

    pub fn user_ids(&self) -> &Vec<i32> {
        &self.user_ids
    }

    pub fn importer_count(&self) -> &Vec<i32> {
        &self.importer_count
    }
}

#[doc(hidden)]
pub struct RTDImportedContactsBuilder {
    inner: ImportedContacts,
}

impl RTDImportedContactsBuilder {
    pub fn build(&self) -> ImportedContacts {
        self.inner.clone()
    }

    pub fn user_ids(&mut self, user_ids: Vec<i32>) -> &mut Self {
        self.inner.user_ids = user_ids;
        self
    }

    pub fn importer_count(&mut self, importer_count: Vec<i32>) -> &mut Self {
        self.inner.importer_count = importer_count;
        self
    }
}

impl AsRef<ImportedContacts> for ImportedContacts {
    fn as_ref(&self) -> &ImportedContacts {
        self
    }
}

impl AsRef<ImportedContacts> for RTDImportedContactsBuilder {
    fn as_ref(&self) -> &ImportedContacts {
        &self.inner
    }
}
