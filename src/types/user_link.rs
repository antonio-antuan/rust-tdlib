use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains an HTTPS URL, which can be used to get information about a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The URL

    #[serde(default)]
    url: String,
    /// Left time for which the link is valid, in seconds; 0 if the link is a public username link

    #[serde(default)]
    expires_in: i32,
}

impl RObject for UserLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl UserLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UserLinkBuilder {
        let mut inner = UserLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UserLinkBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn expires_in(&self) -> i32 {
        self.expires_in
    }
}

#[doc(hidden)]
pub struct UserLinkBuilder {
    inner: UserLink,
}

#[deprecated]
pub type RTDUserLinkBuilder = UserLinkBuilder;

impl UserLinkBuilder {
    pub fn build(&self) -> UserLink {
        self.inner.clone()
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }

    pub fn expires_in(&mut self, expires_in: i32) -> &mut Self {
        self.inner.expires_in = expires_in;
        self
    }
}

impl AsRef<UserLink> for UserLink {
    fn as_ref(&self) -> &UserLink {
        self
    }
}

impl AsRef<UserLink> for UserLinkBuilder {
    fn as_ref(&self) -> &UserLink {
        &self.inner
    }
}
