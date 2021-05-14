use crate::errors::*;
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetContactsBuilder {
        let mut inner = GetContacts::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getContacts".to_string();

        RTDGetContactsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDGetContactsBuilder {
    inner: GetContacts,
}

impl RTDGetContactsBuilder {
    pub fn build(&self) -> GetContacts {
        self.inner.clone()
    }
}

impl AsRef<GetContacts> for GetContacts {
    fn as_ref(&self) -> &GetContacts {
        self
    }
}

impl AsRef<GetContacts> for RTDGetContactsBuilder {
    fn as_ref(&self) -> &GetContacts {
        &self.inner
    }
}
