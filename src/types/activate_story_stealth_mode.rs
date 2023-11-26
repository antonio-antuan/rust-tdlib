use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Activates stealth mode for stories, which hides all views of stories from the current user in the last "story_stealth_mode_past_period" seconds and for the next "story_stealth_mode_future_period" seconds; for Telegram Premium users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActivateStoryStealthMode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ActivateStoryStealthMode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ActivateStoryStealthMode {}

impl ActivateStoryStealthMode {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ActivateStoryStealthModeBuilder {
        let mut inner = ActivateStoryStealthMode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "activateStoryStealthMode".to_string();

        ActivateStoryStealthModeBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ActivateStoryStealthModeBuilder {
    inner: ActivateStoryStealthMode,
}

#[deprecated]
pub type RTDActivateStoryStealthModeBuilder = ActivateStoryStealthModeBuilder;

impl ActivateStoryStealthModeBuilder {
    pub fn build(&self) -> ActivateStoryStealthMode {
        self.inner.clone()
    }
}

impl AsRef<ActivateStoryStealthMode> for ActivateStoryStealthMode {
    fn as_ref(&self) -> &ActivateStoryStealthMode {
        self
    }
}

impl AsRef<ActivateStoryStealthMode> for ActivateStoryStealthModeBuilder {
    fn as_ref(&self) -> &ActivateStoryStealthMode {
        &self.inner
    }
}
