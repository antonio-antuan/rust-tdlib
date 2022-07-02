use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a basic group by its identifier. This is an offline request if the current user is not a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBasicGroup {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Basic group identifier

    #[serde(default)]
    basic_group_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetBasicGroup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetBasicGroup {}

impl GetBasicGroup {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetBasicGroupBuilder {
        let mut inner = GetBasicGroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getBasicGroup".to_string();

        GetBasicGroupBuilder { inner }
    }

    pub fn basic_group_id(&self) -> i64 {
        self.basic_group_id
    }
}

#[doc(hidden)]
pub struct GetBasicGroupBuilder {
    inner: GetBasicGroup,
}

#[deprecated]
pub type RTDGetBasicGroupBuilder = GetBasicGroupBuilder;

impl GetBasicGroupBuilder {
    pub fn build(&self) -> GetBasicGroup {
        self.inner.clone()
    }

    pub fn basic_group_id(&mut self, basic_group_id: i64) -> &mut Self {
        self.inner.basic_group_id = basic_group_id;
        self
    }
}

impl AsRef<GetBasicGroup> for GetBasicGroup {
    fn as_ref(&self) -> &GetBasicGroup {
        self
    }
}

impl AsRef<GetBasicGroup> for GetBasicGroupBuilder {
    fn as_ref(&self) -> &GetBasicGroup {
        &self.inner
    }
}
