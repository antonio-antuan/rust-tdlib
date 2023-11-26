use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns all close friends of the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetCloseFriends {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetCloseFriends {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetCloseFriends {}

impl GetCloseFriends {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetCloseFriendsBuilder {
        let mut inner = GetCloseFriends::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getCloseFriends".to_string();

        GetCloseFriendsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetCloseFriendsBuilder {
    inner: GetCloseFriends,
}

#[deprecated]
pub type RTDGetCloseFriendsBuilder = GetCloseFriendsBuilder;

impl GetCloseFriendsBuilder {
    pub fn build(&self) -> GetCloseFriends {
        self.inner.clone()
    }
}

impl AsRef<GetCloseFriends> for GetCloseFriends {
    fn as_ref(&self) -> &GetCloseFriends {
        self
    }
}

impl AsRef<GetCloseFriends> for GetCloseFriendsBuilder {
    fn as_ref(&self) -> &GetCloseFriends {
        &self.inner
    }
}
