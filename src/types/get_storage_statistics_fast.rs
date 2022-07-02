use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Quickly returns approximate storage usage statistics. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetStorageStatisticsFast {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetStorageStatisticsFast {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetStorageStatisticsFast {}

impl GetStorageStatisticsFast {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetStorageStatisticsFastBuilder {
        let mut inner = GetStorageStatisticsFast::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getStorageStatisticsFast".to_string();

        GetStorageStatisticsFastBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetStorageStatisticsFastBuilder {
    inner: GetStorageStatisticsFast,
}

#[deprecated]
pub type RTDGetStorageStatisticsFastBuilder = GetStorageStatisticsFastBuilder;

impl GetStorageStatisticsFastBuilder {
    pub fn build(&self) -> GetStorageStatisticsFast {
        self.inner.clone()
    }
}

impl AsRef<GetStorageStatisticsFast> for GetStorageStatisticsFast {
    fn as_ref(&self) -> &GetStorageStatisticsFast {
        self
    }
}

impl AsRef<GetStorageStatisticsFast> for GetStorageStatisticsFastBuilder {
    fn as_ref(&self) -> &GetStorageStatisticsFast {
        &self.inner
    }
}
