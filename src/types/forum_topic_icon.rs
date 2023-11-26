use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a forum topic icon
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ForumTopicIcon {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Color of the topic icon in RGB format

    #[serde(default)]
    color: i32,
    /// Unique identifier of the custom emoji shown on the topic icon; 0 if none

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    custom_emoji_id: i64,
}

impl RObject for ForumTopicIcon {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ForumTopicIcon {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ForumTopicIconBuilder {
        let mut inner = ForumTopicIcon::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ForumTopicIconBuilder { inner }
    }

    pub fn color(&self) -> i32 {
        self.color
    }

    pub fn custom_emoji_id(&self) -> i64 {
        self.custom_emoji_id
    }
}

#[doc(hidden)]
pub struct ForumTopicIconBuilder {
    inner: ForumTopicIcon,
}

#[deprecated]
pub type RTDForumTopicIconBuilder = ForumTopicIconBuilder;

impl ForumTopicIconBuilder {
    pub fn build(&self) -> ForumTopicIcon {
        self.inner.clone()
    }

    pub fn color(&mut self, color: i32) -> &mut Self {
        self.inner.color = color;
        self
    }

    pub fn custom_emoji_id(&mut self, custom_emoji_id: i64) -> &mut Self {
        self.inner.custom_emoji_id = custom_emoji_id;
        self
    }
}

impl AsRef<ForumTopicIcon> for ForumTopicIcon {
    fn as_ref(&self) -> &ForumTopicIcon {
        self
    }
}

impl AsRef<ForumTopicIcon> for ForumTopicIconBuilder {
    fn as_ref(&self) -> &ForumTopicIcon {
        &self.inner
    }
}
