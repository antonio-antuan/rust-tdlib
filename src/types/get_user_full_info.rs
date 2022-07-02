use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns full information about a user by their identifier
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetUserFullInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifier

    #[serde(default)]
    user_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetUserFullInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetUserFullInfo {}

impl GetUserFullInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetUserFullInfoBuilder {
        let mut inner = GetUserFullInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getUserFullInfo".to_string();

        GetUserFullInfoBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}

#[doc(hidden)]
pub struct GetUserFullInfoBuilder {
    inner: GetUserFullInfo,
}

#[deprecated]
pub type RTDGetUserFullInfoBuilder = GetUserFullInfoBuilder;

impl GetUserFullInfoBuilder {
    pub fn build(&self) -> GetUserFullInfo {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }
}

impl AsRef<GetUserFullInfo> for GetUserFullInfo {
    fn as_ref(&self) -> &GetUserFullInfo {
        self
    }
}

impl AsRef<GetUserFullInfo> for GetUserFullInfoBuilder {
    fn as_ref(&self) -> &GetUserFullInfo {
        &self.inner
    }
}
