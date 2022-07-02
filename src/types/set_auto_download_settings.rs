use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sets auto-download settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetAutoDownloadSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// New user auto-download settings
    settings: AutoDownloadSettings,
    /// Type of the network for which the new settings are relevant

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "NetworkType::_is_default")]
    type_: NetworkType,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetAutoDownloadSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetAutoDownloadSettings {}

impl SetAutoDownloadSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetAutoDownloadSettingsBuilder {
        let mut inner = SetAutoDownloadSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setAutoDownloadSettings".to_string();

        SetAutoDownloadSettingsBuilder { inner }
    }

    pub fn settings(&self) -> &AutoDownloadSettings {
        &self.settings
    }

    pub fn type_(&self) -> &NetworkType {
        &self.type_
    }
}

#[doc(hidden)]
pub struct SetAutoDownloadSettingsBuilder {
    inner: SetAutoDownloadSettings,
}

#[deprecated]
pub type RTDSetAutoDownloadSettingsBuilder = SetAutoDownloadSettingsBuilder;

impl SetAutoDownloadSettingsBuilder {
    pub fn build(&self) -> SetAutoDownloadSettings {
        self.inner.clone()
    }

    pub fn settings<T: AsRef<AutoDownloadSettings>>(&mut self, settings: T) -> &mut Self {
        self.inner.settings = settings.as_ref().clone();
        self
    }

    pub fn type_<T: AsRef<NetworkType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }
}

impl AsRef<SetAutoDownloadSettings> for SetAutoDownloadSettings {
    fn as_ref(&self) -> &SetAutoDownloadSettings {
        self
    }
}

impl AsRef<SetAutoDownloadSettings> for SetAutoDownloadSettingsBuilder {
    fn as_ref(&self) -> &SetAutoDownloadSettings {
        &self.inner
    }
}
