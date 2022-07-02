use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Toggles whether current user's video is paused
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleGroupCallIsMyVideoPaused {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group call identifier

    #[serde(default)]
    group_call_id: i32,
    /// Pass true if the current user's video is paused

    #[serde(default)]
    is_my_video_paused: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleGroupCallIsMyVideoPaused {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleGroupCallIsMyVideoPaused {}

impl ToggleGroupCallIsMyVideoPaused {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ToggleGroupCallIsMyVideoPausedBuilder {
        let mut inner = ToggleGroupCallIsMyVideoPaused::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleGroupCallIsMyVideoPaused".to_string();

        ToggleGroupCallIsMyVideoPausedBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }

    pub fn is_my_video_paused(&self) -> bool {
        self.is_my_video_paused
    }
}

#[doc(hidden)]
pub struct ToggleGroupCallIsMyVideoPausedBuilder {
    inner: ToggleGroupCallIsMyVideoPaused,
}

#[deprecated]
pub type RTDToggleGroupCallIsMyVideoPausedBuilder = ToggleGroupCallIsMyVideoPausedBuilder;

impl ToggleGroupCallIsMyVideoPausedBuilder {
    pub fn build(&self) -> ToggleGroupCallIsMyVideoPaused {
        self.inner.clone()
    }

    pub fn group_call_id(&mut self, group_call_id: i32) -> &mut Self {
        self.inner.group_call_id = group_call_id;
        self
    }

    pub fn is_my_video_paused(&mut self, is_my_video_paused: bool) -> &mut Self {
        self.inner.is_my_video_paused = is_my_video_paused;
        self
    }
}

impl AsRef<ToggleGroupCallIsMyVideoPaused> for ToggleGroupCallIsMyVideoPaused {
    fn as_ref(&self) -> &ToggleGroupCallIsMyVideoPaused {
        self
    }
}

impl AsRef<ToggleGroupCallIsMyVideoPaused> for ToggleGroupCallIsMyVideoPausedBuilder {
    fn as_ref(&self) -> &ToggleGroupCallIsMyVideoPaused {
        &self.inner
    }
}
