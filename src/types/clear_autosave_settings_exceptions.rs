use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Clears the list of all autosave settings exceptions. The method is guaranteed to work only after at least one call to getAutosaveSettings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClearAutosaveSettingsExceptions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ClearAutosaveSettingsExceptions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ClearAutosaveSettingsExceptions {}

impl ClearAutosaveSettingsExceptions {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ClearAutosaveSettingsExceptionsBuilder {
        let mut inner = ClearAutosaveSettingsExceptions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "clearAutosaveSettingsExceptions".to_string();

        ClearAutosaveSettingsExceptionsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ClearAutosaveSettingsExceptionsBuilder {
    inner: ClearAutosaveSettingsExceptions,
}

#[deprecated]
pub type RTDClearAutosaveSettingsExceptionsBuilder = ClearAutosaveSettingsExceptionsBuilder;

impl ClearAutosaveSettingsExceptionsBuilder {
    pub fn build(&self) -> ClearAutosaveSettingsExceptions {
        self.inner.clone()
    }
}

impl AsRef<ClearAutosaveSettingsExceptions> for ClearAutosaveSettingsExceptions {
    fn as_ref(&self) -> &ClearAutosaveSettingsExceptions {
        self
    }
}

impl AsRef<ClearAutosaveSettingsExceptions> for ClearAutosaveSettingsExceptionsBuilder {
    fn as_ref(&self) -> &ClearAutosaveSettingsExceptions {
        &self.inner
    }
}
