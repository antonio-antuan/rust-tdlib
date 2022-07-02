use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Pauses or unpauses screen sharing in a joined group call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleGroupCallScreenSharingIsPaused {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group call identifier

    #[serde(default)]
    group_call_id: i32,
    /// True if screen sharing is paused

    #[serde(default)]
    is_paused: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleGroupCallScreenSharingIsPaused {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleGroupCallScreenSharingIsPaused {}

impl ToggleGroupCallScreenSharingIsPaused {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ToggleGroupCallScreenSharingIsPausedBuilder {
        let mut inner = ToggleGroupCallScreenSharingIsPaused::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleGroupCallScreenSharingIsPaused".to_string();

        ToggleGroupCallScreenSharingIsPausedBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }
}

#[doc(hidden)]
pub struct ToggleGroupCallScreenSharingIsPausedBuilder {
    inner: ToggleGroupCallScreenSharingIsPaused,
}

#[deprecated]
pub type RTDToggleGroupCallScreenSharingIsPausedBuilder =
    ToggleGroupCallScreenSharingIsPausedBuilder;

impl ToggleGroupCallScreenSharingIsPausedBuilder {
    pub fn build(&self) -> ToggleGroupCallScreenSharingIsPaused {
        self.inner.clone()
    }

    pub fn group_call_id(&mut self, group_call_id: i32) -> &mut Self {
        self.inner.group_call_id = group_call_id;
        self
    }

    pub fn is_paused(&mut self, is_paused: bool) -> &mut Self {
        self.inner.is_paused = is_paused;
        self
    }
}

impl AsRef<ToggleGroupCallScreenSharingIsPaused> for ToggleGroupCallScreenSharingIsPaused {
    fn as_ref(&self) -> &ToggleGroupCallScreenSharingIsPaused {
        self
    }
}

impl AsRef<ToggleGroupCallScreenSharingIsPaused> for ToggleGroupCallScreenSharingIsPausedBuilder {
    fn as_ref(&self) -> &ToggleGroupCallScreenSharingIsPaused {
        &self.inner
    }
}
