use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns auto-download settings presets for the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAutoDownloadSettingsPresets {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetAutoDownloadSettingsPresets {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetAutoDownloadSettingsPresets {}

impl GetAutoDownloadSettingsPresets {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetAutoDownloadSettingsPresetsBuilder {
        let mut inner = GetAutoDownloadSettingsPresets::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getAutoDownloadSettingsPresets".to_string();

        GetAutoDownloadSettingsPresetsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetAutoDownloadSettingsPresetsBuilder {
    inner: GetAutoDownloadSettingsPresets,
}

#[deprecated]
pub type RTDGetAutoDownloadSettingsPresetsBuilder = GetAutoDownloadSettingsPresetsBuilder;

impl GetAutoDownloadSettingsPresetsBuilder {
    pub fn build(&self) -> GetAutoDownloadSettingsPresets {
        self.inner.clone()
    }
}

impl AsRef<GetAutoDownloadSettingsPresets> for GetAutoDownloadSettingsPresets {
    fn as_ref(&self) -> &GetAutoDownloadSettingsPresets {
        self
    }
}

impl AsRef<GetAutoDownloadSettingsPresets> for GetAutoDownloadSettingsPresetsBuilder {
    fn as_ref(&self) -> &GetAutoDownloadSettingsPresets {
        &self.inner
    }
}
