use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns support information for the given user; for Telegram support only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetUserSupportInfo {
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

impl RObject for GetUserSupportInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetUserSupportInfo {}

impl GetUserSupportInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetUserSupportInfoBuilder {
        let mut inner = GetUserSupportInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getUserSupportInfo".to_string();

        GetUserSupportInfoBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}

#[doc(hidden)]
pub struct GetUserSupportInfoBuilder {
    inner: GetUserSupportInfo,
}

#[deprecated]
pub type RTDGetUserSupportInfoBuilder = GetUserSupportInfoBuilder;

impl GetUserSupportInfoBuilder {
    pub fn build(&self) -> GetUserSupportInfo {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }
}

impl AsRef<GetUserSupportInfo> for GetUserSupportInfo {
    fn as_ref(&self) -> &GetUserSupportInfo {
        self
    }
}

impl AsRef<GetUserSupportInfo> for GetUserSupportInfoBuilder {
    fn as_ref(&self) -> &GetUserSupportInfo {
        &self.inner
    }
}
