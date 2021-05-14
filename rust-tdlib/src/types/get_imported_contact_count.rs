use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns the total number of imported contacts
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetImportedContactCount {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetImportedContactCount {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetImportedContactCount {}

impl GetImportedContactCount {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetImportedContactCountBuilder {
        let mut inner = GetImportedContactCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getImportedContactCount".to_string();

        RTDGetImportedContactCountBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDGetImportedContactCountBuilder {
    inner: GetImportedContactCount,
}

impl RTDGetImportedContactCountBuilder {
    pub fn build(&self) -> GetImportedContactCount {
        self.inner.clone()
    }
}

impl AsRef<GetImportedContactCount> for GetImportedContactCount {
    fn as_ref(&self) -> &GetImportedContactCount {
        self
    }
}

impl AsRef<GetImportedContactCount> for RTDGetImportedContactCountBuilder {
    fn as_ref(&self) -> &GetImportedContactCount {
        &self.inner
    }
}
