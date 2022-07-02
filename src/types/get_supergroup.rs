use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a supergroup or a channel by its identifier. This is an offline request if the current user is not a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSupergroup {
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

impl RObject for GetSupergroup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetSupergroup {}

impl GetSupergroup {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetSupergroupBuilder {
        let mut inner = GetSupergroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getSupergroup".to_string();

        GetSupergroupBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i64 {
        self.supergroup_id
    }
}

#[doc(hidden)]
pub struct GetSupergroupBuilder {
    inner: GetSupergroup,
}

#[deprecated]
pub type RTDGetSupergroupBuilder = GetSupergroupBuilder;

impl GetSupergroupBuilder {
    pub fn build(&self) -> GetSupergroup {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
        self.inner.supergroup_id = supergroup_id;
        self
    }
}

impl AsRef<GetSupergroup> for GetSupergroup {
    fn as_ref(&self) -> &GetSupergroup {
        self
    }
}

impl AsRef<GetSupergroup> for GetSupergroupBuilder {
    fn as_ref(&self) -> &GetSupergroup {
        &self.inner
    }
}
