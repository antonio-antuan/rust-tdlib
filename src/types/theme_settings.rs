use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes theme settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThemeSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Theme accent color in ARGB format

    #[serde(default)]
    accent_color: i32,
    /// The background to be used in chats; may be null
    background: Option<Background>,
    /// The fill to be used as a background for outgoing messages

    #[serde(skip_serializing_if = "BackgroundFill::_is_default")]
    outgoing_message_fill: BackgroundFill,
    /// If true, the freeform gradient fill needs to be animated on every sent message

    #[serde(default)]
    animate_outgoing_message_fill: bool,
    /// Accent color of outgoing messages in ARGB format

    #[serde(default)]
    outgoing_message_accent_color: i32,
}

impl RObject for ThemeSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ThemeSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ThemeSettingsBuilder {
        let mut inner = ThemeSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ThemeSettingsBuilder { inner }
    }

    pub fn accent_color(&self) -> i32 {
        self.accent_color
    }

    pub fn background(&self) -> &Option<Background> {
        &self.background
    }

    pub fn outgoing_message_fill(&self) -> &BackgroundFill {
        &self.outgoing_message_fill
    }

    pub fn animate_outgoing_message_fill(&self) -> bool {
        self.animate_outgoing_message_fill
    }

    pub fn outgoing_message_accent_color(&self) -> i32 {
        self.outgoing_message_accent_color
    }
}

#[doc(hidden)]
pub struct ThemeSettingsBuilder {
    inner: ThemeSettings,
}

#[deprecated]
pub type RTDThemeSettingsBuilder = ThemeSettingsBuilder;

impl ThemeSettingsBuilder {
    pub fn build(&self) -> ThemeSettings {
        self.inner.clone()
    }

    pub fn accent_color(&mut self, accent_color: i32) -> &mut Self {
        self.inner.accent_color = accent_color;
        self
    }

    pub fn background<T: AsRef<Background>>(&mut self, background: T) -> &mut Self {
        self.inner.background = Some(background.as_ref().clone());
        self
    }

    pub fn outgoing_message_fill<T: AsRef<BackgroundFill>>(
        &mut self,
        outgoing_message_fill: T,
    ) -> &mut Self {
        self.inner.outgoing_message_fill = outgoing_message_fill.as_ref().clone();
        self
    }

    pub fn animate_outgoing_message_fill(
        &mut self,
        animate_outgoing_message_fill: bool,
    ) -> &mut Self {
        self.inner.animate_outgoing_message_fill = animate_outgoing_message_fill;
        self
    }

    pub fn outgoing_message_accent_color(
        &mut self,
        outgoing_message_accent_color: i32,
    ) -> &mut Self {
        self.inner.outgoing_message_accent_color = outgoing_message_accent_color;
        self
    }
}

impl AsRef<ThemeSettings> for ThemeSettings {
    fn as_ref(&self) -> &ThemeSettings {
        self
    }
}

impl AsRef<ThemeSettings> for ThemeSettingsBuilder {
    fn as_ref(&self) -> &ThemeSettings {
        &self.inner
    }
}
