use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a chat theme
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatTheme {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Theme name

    #[serde(default)]
    name: String,
    /// Theme settings for a light chat theme
    light_settings: ThemeSettings,
    /// Theme settings for a dark chat theme
    dark_settings: ThemeSettings,
}

impl RObject for ChatTheme {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatTheme {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatThemeBuilder {
        let mut inner = ChatTheme::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatThemeBuilder { inner }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn light_settings(&self) -> &ThemeSettings {
        &self.light_settings
    }

    pub fn dark_settings(&self) -> &ThemeSettings {
        &self.dark_settings
    }
}

#[doc(hidden)]
pub struct ChatThemeBuilder {
    inner: ChatTheme,
}

#[deprecated]
pub type RTDChatThemeBuilder = ChatThemeBuilder;

impl ChatThemeBuilder {
    pub fn build(&self) -> ChatTheme {
        self.inner.clone()
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }

    pub fn light_settings<T: AsRef<ThemeSettings>>(&mut self, light_settings: T) -> &mut Self {
        self.inner.light_settings = light_settings.as_ref().clone();
        self
    }

    pub fn dark_settings<T: AsRef<ThemeSettings>>(&mut self, dark_settings: T) -> &mut Self {
        self.inner.dark_settings = dark_settings.as_ref().clone();
        self
    }
}

impl AsRef<ChatTheme> for ChatTheme {
    fn as_ref(&self) -> &ChatTheme {
        self
    }
}

impl AsRef<ChatTheme> for ChatThemeBuilder {
    fn as_ref(&self) -> &ChatTheme {
        &self.inner
    }
}
