use crate::errors::*;
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
    user_id: i32,

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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetUserFullInfoBuilder {
        let mut inner = GetUserFullInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getUserFullInfo".to_string();

        RTDGetUserFullInfoBuilder { inner }
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }
}

#[doc(hidden)]
pub struct RTDGetUserFullInfoBuilder {
    inner: GetUserFullInfo,
}

impl RTDGetUserFullInfoBuilder {
    pub fn build(&self) -> GetUserFullInfo {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }
}

impl AsRef<GetUserFullInfo> for GetUserFullInfo {
    fn as_ref(&self) -> &GetUserFullInfo {
        self
    }
}

impl AsRef<GetUserFullInfo> for RTDGetUserFullInfoBuilder {
    fn as_ref(&self) -> &GetUserFullInfo {
        &self.inner
    }
}
