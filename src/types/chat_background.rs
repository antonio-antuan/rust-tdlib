use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a background set for a specific chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatBackground {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The background
    background: Background,
    /// Dimming of the background in dark themes, as a percentage; 0-100

    #[serde(default)]
    dark_theme_dimming: i32,
}

impl RObject for ChatBackground {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatBackground {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatBackgroundBuilder {
        let mut inner = ChatBackground::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatBackgroundBuilder { inner }
    }

    pub fn background(&self) -> &Background {
        &self.background
    }

    pub fn dark_theme_dimming(&self) -> i32 {
        self.dark_theme_dimming
    }
}

#[doc(hidden)]
pub struct ChatBackgroundBuilder {
    inner: ChatBackground,
}

#[deprecated]
pub type RTDChatBackgroundBuilder = ChatBackgroundBuilder;

impl ChatBackgroundBuilder {
    pub fn build(&self) -> ChatBackground {
        self.inner.clone()
    }

    pub fn background<T: AsRef<Background>>(&mut self, background: T) -> &mut Self {
        self.inner.background = background.as_ref().clone();
        self
    }

    pub fn dark_theme_dimming(&mut self, dark_theme_dimming: i32) -> &mut Self {
        self.inner.dark_theme_dimming = dark_theme_dimming;
        self
    }
}

impl AsRef<ChatBackground> for ChatBackground {
    fn as_ref(&self) -> &ChatBackground {
        self
    }
}

impl AsRef<ChatBackground> for ChatBackgroundBuilder {
    fn as_ref(&self) -> &ChatBackground {
        &self.inner
    }
}
