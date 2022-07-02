use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of users
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Users {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Approximate total count of users found

    #[serde(default)]
    total_count: i32,
    /// A list of user identifiers

    #[serde(default)]
    user_ids: Vec<i64>,
}

impl RObject for Users {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Users {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UsersBuilder {
        let mut inner = Users::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UsersBuilder { inner }
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn user_ids(&self) -> &Vec<i64> {
        &self.user_ids
    }
}

#[doc(hidden)]
pub struct UsersBuilder {
    inner: Users,
}

#[deprecated]
pub type RTDUsersBuilder = UsersBuilder;

impl UsersBuilder {
    pub fn build(&self) -> Users {
        self.inner.clone()
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }

    pub fn user_ids(&mut self, user_ids: Vec<i64>) -> &mut Self {
        self.inner.user_ids = user_ids;
        self
    }
}

impl AsRef<Users> for Users {
    fn as_ref(&self) -> &Users {
        self
    }
}

impl AsRef<Users> for UsersBuilder {
    fn as_ref(&self) -> &Users {
        &self.inner
    }
}
