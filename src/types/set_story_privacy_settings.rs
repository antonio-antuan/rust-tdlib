use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes privacy settings of a story. Can be called only if story.can_be_edited == true
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetStoryPrivacySettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat that posted the story

    #[serde(default)]
    story_sender_chat_id: i64,
    /// Identifier of the story

    #[serde(default)]
    story_id: i32,
    /// The new privacy settigs for the story

    #[serde(skip_serializing_if = "StoryPrivacySettings::_is_default")]
    privacy_settings: StoryPrivacySettings,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetStoryPrivacySettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetStoryPrivacySettings {}

impl SetStoryPrivacySettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetStoryPrivacySettingsBuilder {
        let mut inner = SetStoryPrivacySettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setStoryPrivacySettings".to_string();

        SetStoryPrivacySettingsBuilder { inner }
    }

    pub fn story_sender_chat_id(&self) -> i64 {
        self.story_sender_chat_id
    }

    pub fn story_id(&self) -> i32 {
        self.story_id
    }

    pub fn privacy_settings(&self) -> &StoryPrivacySettings {
        &self.privacy_settings
    }
}

#[doc(hidden)]
pub struct SetStoryPrivacySettingsBuilder {
    inner: SetStoryPrivacySettings,
}

#[deprecated]
pub type RTDSetStoryPrivacySettingsBuilder = SetStoryPrivacySettingsBuilder;

impl SetStoryPrivacySettingsBuilder {
    pub fn build(&self) -> SetStoryPrivacySettings {
        self.inner.clone()
    }

    pub fn story_sender_chat_id(&mut self, story_sender_chat_id: i64) -> &mut Self {
        self.inner.story_sender_chat_id = story_sender_chat_id;
        self
    }

    pub fn story_id(&mut self, story_id: i32) -> &mut Self {
        self.inner.story_id = story_id;
        self
    }

    pub fn privacy_settings<T: AsRef<StoryPrivacySettings>>(
        &mut self,
        privacy_settings: T,
    ) -> &mut Self {
        self.inner.privacy_settings = privacy_settings.as_ref().clone();
        self
    }
}

impl AsRef<SetStoryPrivacySettings> for SetStoryPrivacySettings {
    fn as_ref(&self) -> &SetStoryPrivacySettings {
        self
    }
}

impl AsRef<SetStoryPrivacySettings> for SetStoryPrivacySettingsBuilder {
    fn as_ref(&self) -> &SetStoryPrivacySettings {
        &self.inner
    }
}
