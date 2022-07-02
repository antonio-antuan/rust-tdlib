use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns full information about a supergroup or a channel by its identifier, cached for up to 1 minute
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSupergroupFullInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Supergroup or channel identifier

    #[serde(default)]
    supergroup_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetSupergroupFullInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetSupergroupFullInfo {}

impl GetSupergroupFullInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetSupergroupFullInfoBuilder {
        let mut inner = GetSupergroupFullInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getSupergroupFullInfo".to_string();

        GetSupergroupFullInfoBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i64 {
        self.supergroup_id
    }
}

#[doc(hidden)]
pub struct GetSupergroupFullInfoBuilder {
    inner: GetSupergroupFullInfo,
}

#[deprecated]
pub type RTDGetSupergroupFullInfoBuilder = GetSupergroupFullInfoBuilder;

impl GetSupergroupFullInfoBuilder {
    pub fn build(&self) -> GetSupergroupFullInfo {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
        self.inner.supergroup_id = supergroup_id;
        self
    }
}

impl AsRef<GetSupergroupFullInfo> for GetSupergroupFullInfo {
    fn as_ref(&self) -> &GetSupergroupFullInfo {
        self
    }
}

impl AsRef<GetSupergroupFullInfo> for GetSupergroupFullInfoBuilder {
    fn as_ref(&self) -> &GetSupergroupFullInfo {
        &self.inner
    }
}
