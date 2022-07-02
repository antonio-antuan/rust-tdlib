use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a user by their identifier. This is an offline request if the current user is not a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetUser {
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

impl RObject for GetUser {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetUser {}

impl GetUser {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetUserBuilder {
        let mut inner = GetUser::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getUser".to_string();

        GetUserBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}

#[doc(hidden)]
pub struct GetUserBuilder {
    inner: GetUser,
}

#[deprecated]
pub type RTDGetUserBuilder = GetUserBuilder;

impl GetUserBuilder {
    pub fn build(&self) -> GetUser {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }
}

impl AsRef<GetUser> for GetUser {
    fn as_ref(&self) -> &GetUser {
        self
    }
}

impl AsRef<GetUser> for GetUserBuilder {
    fn as_ref(&self) -> &GetUser {
        &self.inner
    }
}
