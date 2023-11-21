use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns localized name of the Telegram support user; for Telegram support only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSupportName {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetSupportName {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetSupportName {}

impl GetSupportName {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetSupportNameBuilder {
        let mut inner = GetSupportName::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getSupportName".to_string();

        GetSupportNameBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetSupportNameBuilder {
    inner: GetSupportName,
}

#[deprecated]
pub type RTDGetSupportNameBuilder = GetSupportNameBuilder;

impl GetSupportNameBuilder {
    pub fn build(&self) -> GetSupportName {
        self.inner.clone()
    }
}

impl AsRef<GetSupportName> for GetSupportName {
    fn as_ref(&self) -> &GetSupportName {
        self
    }
}

impl AsRef<GetSupportName> for GetSupportNameBuilder {
    fn as_ref(&self) -> &GetSupportName {
        &self.inner
    }
}
