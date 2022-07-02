use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns the period of inactivity after which the account of the current user will automatically be deleted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAccountTtl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetAccountTtl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetAccountTtl {}

impl GetAccountTtl {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetAccountTtlBuilder {
        let mut inner = GetAccountTtl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getAccountTtl".to_string();

        GetAccountTtlBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetAccountTtlBuilder {
    inner: GetAccountTtl,
}

#[deprecated]
pub type RTDGetAccountTtlBuilder = GetAccountTtlBuilder;

impl GetAccountTtlBuilder {
    pub fn build(&self) -> GetAccountTtl {
        self.inner.clone()
    }
}

impl AsRef<GetAccountTtl> for GetAccountTtl {
    fn as_ref(&self) -> &GetAccountTtl {
        self
    }
}

impl AsRef<GetAccountTtl> for GetAccountTtlBuilder {
    fn as_ref(&self) -> &GetAccountTtl {
        &self.inner
    }
}
