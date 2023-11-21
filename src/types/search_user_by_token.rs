use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Searches a user by a token from the user's link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchUserByToken {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Token to search for

    #[serde(default)]
    token: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SearchUserByToken {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SearchUserByToken {}

impl SearchUserByToken {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchUserByTokenBuilder {
        let mut inner = SearchUserByToken::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "searchUserByToken".to_string();

        SearchUserByTokenBuilder { inner }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

#[doc(hidden)]
pub struct SearchUserByTokenBuilder {
    inner: SearchUserByToken,
}

#[deprecated]
pub type RTDSearchUserByTokenBuilder = SearchUserByTokenBuilder;

impl SearchUserByTokenBuilder {
    pub fn build(&self) -> SearchUserByToken {
        self.inner.clone()
    }

    pub fn token<T: AsRef<str>>(&mut self, token: T) -> &mut Self {
        self.inner.token = token.as_ref().to_string();
        self
    }
}

impl AsRef<SearchUserByToken> for SearchUserByToken {
    fn as_ref(&self) -> &SearchUserByToken {
        self
    }
}

impl AsRef<SearchUserByToken> for SearchUserByTokenBuilder {
    fn as_ref(&self) -> &SearchUserByToken {
        &self.inner
    }
}
