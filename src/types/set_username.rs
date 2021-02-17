use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Changes the username of the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetUsername {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The new value of the username. Use an empty string to remove the username
    username: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetUsername {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetUsername {}

impl SetUsername {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetUsernameBuilder {
        let mut inner = SetUsername::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setUsername".to_string();

        RTDSetUsernameBuilder { inner }
    }

    pub fn username(&self) -> &String {
        &self.username
    }
}

#[doc(hidden)]
pub struct RTDSetUsernameBuilder {
    inner: SetUsername,
}

impl RTDSetUsernameBuilder {
    pub fn build(&self) -> SetUsername {
        self.inner.clone()
    }

    pub fn username<T: AsRef<str>>(&mut self, username: T) -> &mut Self {
        self.inner.username = username.as_ref().to_string();
        self
    }
}

impl AsRef<SetUsername> for SetUsername {
    fn as_ref(&self) -> &SetUsername {
        self
    }
}

impl AsRef<SetUsername> for RTDSetUsernameBuilder {
    fn as_ref(&self) -> &SetUsername {
        &self.inner
    }
}
