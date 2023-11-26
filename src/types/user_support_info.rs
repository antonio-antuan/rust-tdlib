use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains custom information about the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserSupportInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Information message
    message: FormattedText,
    /// Information author

    #[serde(default)]
    author: String,
    /// Information change date

    #[serde(default)]
    date: i32,
}

impl RObject for UserSupportInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl UserSupportInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UserSupportInfoBuilder {
        let mut inner = UserSupportInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UserSupportInfoBuilder { inner }
    }

    pub fn message(&self) -> &FormattedText {
        &self.message
    }

    pub fn author(&self) -> &String {
        &self.author
    }

    pub fn date(&self) -> i32 {
        self.date
    }
}

#[doc(hidden)]
pub struct UserSupportInfoBuilder {
    inner: UserSupportInfo,
}

#[deprecated]
pub type RTDUserSupportInfoBuilder = UserSupportInfoBuilder;

impl UserSupportInfoBuilder {
    pub fn build(&self) -> UserSupportInfo {
        self.inner.clone()
    }

    pub fn message<T: AsRef<FormattedText>>(&mut self, message: T) -> &mut Self {
        self.inner.message = message.as_ref().clone();
        self
    }

    pub fn author<T: AsRef<str>>(&mut self, author: T) -> &mut Self {
        self.inner.author = author.as_ref().to_string();
        self
    }

    pub fn date(&mut self, date: i32) -> &mut Self {
        self.inner.date = date;
        self
    }
}

impl AsRef<UserSupportInfo> for UserSupportInfo {
    fn as_ref(&self) -> &UserSupportInfo {
        self
    }
}

impl AsRef<UserSupportInfo> for UserSupportInfoBuilder {
    fn as_ref(&self) -> &UserSupportInfo {
        &self.inner
    }
}
