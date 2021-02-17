use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Clears all imported contacts, contact list remains unchanged
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClearImportedContacts {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ClearImportedContacts {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ClearImportedContacts {}

impl ClearImportedContacts {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDClearImportedContactsBuilder {
        let mut inner = ClearImportedContacts::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "clearImportedContacts".to_string();

        RTDClearImportedContactsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDClearImportedContactsBuilder {
    inner: ClearImportedContacts,
}

impl RTDClearImportedContactsBuilder {
    pub fn build(&self) -> ClearImportedContacts {
        self.inner.clone()
    }
}

impl AsRef<ClearImportedContacts> for ClearImportedContacts {
    fn as_ref(&self) -> &ClearImportedContacts {
        self
    }
}

impl AsRef<ClearImportedContacts> for RTDClearImportedContactsBuilder {
    fn as_ref(&self) -> &ClearImportedContacts {
        &self.inner
    }
}
