use crate::errors::*;
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
    basic_group_id: i32,

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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetBasicGroupBuilder {
        let mut inner = GetBasicGroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getBasicGroup".to_string();

        RTDGetBasicGroupBuilder { inner }
    }

    pub fn basic_group_id(&self) -> i32 {
        self.basic_group_id
    }
}

#[doc(hidden)]
pub struct RTDGetBasicGroupBuilder {
    inner: GetBasicGroup,
}

impl RTDGetBasicGroupBuilder {
    pub fn build(&self) -> GetBasicGroup {
        self.inner.clone()
    }

    pub fn basic_group_id(&mut self, basic_group_id: i32) -> &mut Self {
        self.inner.basic_group_id = basic_group_id;
        self
    }
}

impl AsRef<GetBasicGroup> for GetBasicGroup {
    fn as_ref(&self) -> &GetBasicGroup {
        self
    }
}

impl AsRef<GetBasicGroup> for RTDGetBasicGroupBuilder {
    fn as_ref(&self) -> &GetBasicGroup {
        &self.inner
    }
}
