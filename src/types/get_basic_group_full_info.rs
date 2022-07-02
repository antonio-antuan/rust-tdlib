use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns full information about a basic group by its identifier
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBasicGroupFullInfo {
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

impl RObject for GetBasicGroupFullInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetBasicGroupFullInfo {}

impl GetBasicGroupFullInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetBasicGroupFullInfoBuilder {
        let mut inner = GetBasicGroupFullInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getBasicGroupFullInfo".to_string();

        GetBasicGroupFullInfoBuilder { inner }
    }

    pub fn basic_group_id(&self) -> i64 {
        self.basic_group_id
    }
}

#[doc(hidden)]
pub struct GetBasicGroupFullInfoBuilder {
    inner: GetBasicGroupFullInfo,
}

#[deprecated]
pub type RTDGetBasicGroupFullInfoBuilder = GetBasicGroupFullInfoBuilder;

impl GetBasicGroupFullInfoBuilder {
    pub fn build(&self) -> GetBasicGroupFullInfo {
        self.inner.clone()
    }

    pub fn basic_group_id(&mut self, basic_group_id: i64) -> &mut Self {
        self.inner.basic_group_id = basic_group_id;
        self
    }
}

impl AsRef<GetBasicGroupFullInfo> for GetBasicGroupFullInfo {
    fn as_ref(&self) -> &GetBasicGroupFullInfo {
        self
    }
}

impl AsRef<GetBasicGroupFullInfo> for GetBasicGroupFullInfoBuilder {
    fn as_ref(&self) -> &GetBasicGroupFullInfo {
        &self.inner
    }
}
