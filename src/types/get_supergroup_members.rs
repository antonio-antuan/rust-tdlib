use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about members or banned users in a supergroup or channel. Can be used only if supergroupFullInfo.can_get_members == true; additionally, administrator privileges may be required for some filters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSupergroupMembers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the supergroup or channel

    #[serde(default)]
    supergroup_id: i64,
    /// The type of users to return; pass null to use supergroupMembersFilterRecent

    #[serde(skip_serializing_if = "SupergroupMembersFilter::_is_default")]
    filter: SupergroupMembersFilter,
    /// Number of users to skip

    #[serde(default)]
    offset: i32,
    /// The maximum number of users be returned; up to 200

    #[serde(default)]
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetSupergroupMembers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetSupergroupMembers {}

impl GetSupergroupMembers {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetSupergroupMembersBuilder {
        let mut inner = GetSupergroupMembers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getSupergroupMembers".to_string();

        GetSupergroupMembersBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i64 {
        self.supergroup_id
    }

    pub fn filter(&self) -> &SupergroupMembersFilter {
        &self.filter
    }

    pub fn offset(&self) -> i32 {
        self.offset
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct GetSupergroupMembersBuilder {
    inner: GetSupergroupMembers,
}

#[deprecated]
pub type RTDGetSupergroupMembersBuilder = GetSupergroupMembersBuilder;

impl GetSupergroupMembersBuilder {
    pub fn build(&self) -> GetSupergroupMembers {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
        self.inner.supergroup_id = supergroup_id;
        self
    }

    pub fn filter<T: AsRef<SupergroupMembersFilter>>(&mut self, filter: T) -> &mut Self {
        self.inner.filter = filter.as_ref().clone();
        self
    }

    pub fn offset(&mut self, offset: i32) -> &mut Self {
        self.inner.offset = offset;
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<GetSupergroupMembers> for GetSupergroupMembers {
    fn as_ref(&self) -> &GetSupergroupMembers {
        self
    }
}

impl AsRef<GetSupergroupMembers> for GetSupergroupMembersBuilder {
    fn as_ref(&self) -> &GetSupergroupMembers {
        &self.inner
    }
}
