use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Toggles whether current user's video is enabled
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleGroupCallIsMyVideoEnabled {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group call identifier

    #[serde(default)]
    group_call_id: i32,
    /// Pass true if the current user's video is enabled

    #[serde(default)]
    is_my_video_enabled: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleGroupCallIsMyVideoEnabled {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleGroupCallIsMyVideoEnabled {}

impl ToggleGroupCallIsMyVideoEnabled {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ToggleGroupCallIsMyVideoEnabledBuilder {
        let mut inner = ToggleGroupCallIsMyVideoEnabled::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleGroupCallIsMyVideoEnabled".to_string();

        ToggleGroupCallIsMyVideoEnabledBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }

    pub fn is_my_video_enabled(&self) -> bool {
        self.is_my_video_enabled
    }
}

#[doc(hidden)]
pub struct ToggleGroupCallIsMyVideoEnabledBuilder {
    inner: ToggleGroupCallIsMyVideoEnabled,
}

#[deprecated]
pub type RTDToggleGroupCallIsMyVideoEnabledBuilder = ToggleGroupCallIsMyVideoEnabledBuilder;

impl ToggleGroupCallIsMyVideoEnabledBuilder {
    pub fn build(&self) -> ToggleGroupCallIsMyVideoEnabled {
        self.inner.clone()
    }

    pub fn group_call_id(&mut self, group_call_id: i32) -> &mut Self {
        self.inner.group_call_id = group_call_id;
        self
    }

    pub fn is_my_video_enabled(&mut self, is_my_video_enabled: bool) -> &mut Self {
        self.inner.is_my_video_enabled = is_my_video_enabled;
        self
    }
}

impl AsRef<ToggleGroupCallIsMyVideoEnabled> for ToggleGroupCallIsMyVideoEnabled {
    fn as_ref(&self) -> &ToggleGroupCallIsMyVideoEnabled {
        self
    }
}

impl AsRef<ToggleGroupCallIsMyVideoEnabled> for ToggleGroupCallIsMyVideoEnabledBuilder {
    fn as_ref(&self) -> &ToggleGroupCallIsMyVideoEnabled {
        &self.inner
    }
}
