use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sets support information for the given user; for Telegram support only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetUserSupportInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifier

    #[serde(default)]
    user_id: i64,
    /// New information message
    message: FormattedText,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetUserSupportInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetUserSupportInfo {}

impl SetUserSupportInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetUserSupportInfoBuilder {
        let mut inner = SetUserSupportInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setUserSupportInfo".to_string();

        SetUserSupportInfoBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn message(&self) -> &FormattedText {
        &self.message
    }
}

#[doc(hidden)]
pub struct SetUserSupportInfoBuilder {
    inner: SetUserSupportInfo,
}

#[deprecated]
pub type RTDSetUserSupportInfoBuilder = SetUserSupportInfoBuilder;

impl SetUserSupportInfoBuilder {
    pub fn build(&self) -> SetUserSupportInfo {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn message<T: AsRef<FormattedText>>(&mut self, message: T) -> &mut Self {
        self.inner.message = message.as_ref().clone();
        self
    }
}

impl AsRef<SetUserSupportInfo> for SetUserSupportInfo {
    fn as_ref(&self) -> &SetUserSupportInfo {
        self
    }
}

impl AsRef<SetUserSupportInfo> for SetUserSupportInfoBuilder {
    fn as_ref(&self) -> &SetUserSupportInfo {
        &self.inner
    }
}
