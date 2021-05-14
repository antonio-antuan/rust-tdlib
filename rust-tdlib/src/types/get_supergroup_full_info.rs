use crate::errors::*;
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
    supergroup_id: i32,

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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetSupergroupFullInfoBuilder {
        let mut inner = GetSupergroupFullInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getSupergroupFullInfo".to_string();

        RTDGetSupergroupFullInfoBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i32 {
        self.supergroup_id
    }
}

#[doc(hidden)]
pub struct RTDGetSupergroupFullInfoBuilder {
    inner: GetSupergroupFullInfo,
}

impl RTDGetSupergroupFullInfoBuilder {
    pub fn build(&self) -> GetSupergroupFullInfo {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self {
        self.inner.supergroup_id = supergroup_id;
        self
    }
}

impl AsRef<GetSupergroupFullInfo> for GetSupergroupFullInfo {
    fn as_ref(&self) -> &GetSupergroupFullInfo {
        self
    }
}

impl AsRef<GetSupergroupFullInfo> for RTDGetSupergroupFullInfoBuilder {
    fn as_ref(&self) -> &GetSupergroupFullInfo {
        &self.inner
    }
}
