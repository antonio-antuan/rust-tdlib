use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns all user contacts
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetContacts {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetContacts {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetContacts {}

impl GetContacts {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetContactsBuilder {
        let mut inner = GetContacts::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getContacts".to_string();

        GetContactsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetContactsBuilder {
    inner: GetContacts,
}

#[deprecated]
pub type RTDGetContactsBuilder = GetContactsBuilder;

impl GetContactsBuilder {
    pub fn build(&self) -> GetContacts {
        self.inner.clone()
    }
}

impl AsRef<GetContacts> for GetContacts {
    fn as_ref(&self) -> &GetContacts {
        self
    }
}

impl AsRef<GetContacts> for GetContactsBuilder {
    fn as_ref(&self) -> &GetContacts {
        &self.inner
    }
}
