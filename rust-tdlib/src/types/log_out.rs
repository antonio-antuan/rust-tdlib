use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Closes the TDLib instance after a proper logout. Requires an available network connection. All local data will be destroyed. After the logout completes, updateAuthorizationState with authorizationStateClosed will be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogOut {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for LogOut {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for LogOut {}

impl LogOut {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDLogOutBuilder {
        let mut inner = LogOut::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "logOut".to_string();

        RTDLogOutBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDLogOutBuilder {
    inner: LogOut,
}

impl RTDLogOutBuilder {
    pub fn build(&self) -> LogOut {
        self.inner.clone()
    }
}

impl AsRef<LogOut> for LogOut {
    fn as_ref(&self) -> &LogOut {
        self
    }
}

impl AsRef<LogOut> for RTDLogOutBuilder {
    fn as_ref(&self) -> &LogOut {
        &self.inner
    }
}
