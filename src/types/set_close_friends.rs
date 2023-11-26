use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the list of close friends of the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetCloseFriends {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifiers of close friends; the users must be contacts of the current user

    #[serde(default)]
    user_ids: Vec<i64>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetCloseFriends {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetCloseFriends {}

impl SetCloseFriends {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetCloseFriendsBuilder {
        let mut inner = SetCloseFriends::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setCloseFriends".to_string();

        SetCloseFriendsBuilder { inner }
    }

    pub fn user_ids(&self) -> &Vec<i64> {
        &self.user_ids
    }
}

#[doc(hidden)]
pub struct SetCloseFriendsBuilder {
    inner: SetCloseFriends,
}

#[deprecated]
pub type RTDSetCloseFriendsBuilder = SetCloseFriendsBuilder;

impl SetCloseFriendsBuilder {
    pub fn build(&self) -> SetCloseFriends {
        self.inner.clone()
    }

    pub fn user_ids(&mut self, user_ids: Vec<i64>) -> &mut Self {
        self.inner.user_ids = user_ids;
        self
    }
}

impl AsRef<SetCloseFriends> for SetCloseFriends {
    fn as_ref(&self) -> &SetCloseFriends {
        self
    }
}

impl AsRef<SetCloseFriends> for SetCloseFriendsBuilder {
    fn as_ref(&self) -> &SetCloseFriends {
        &self.inner
    }
}
