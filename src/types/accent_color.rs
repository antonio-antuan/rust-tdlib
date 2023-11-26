use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about supported accent color for user/chat name, background of empty chat photo, replies to messages and link previews
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccentColor {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Accent color identifier

    #[serde(default)]
    id: i32,
    /// Identifier of a built-in color to use in places, where only one color is needed; 0-6

    #[serde(default)]
    built_in_accent_color_id: i32,
    /// The list of 1-3 colors in RGB format, describing the accent color, as expected to be shown in light themes

    #[serde(default)]
    light_theme_colors: Vec<i32>,
    /// The list of 1-3 colors in RGB format, describing the accent color, as expected to be shown in dark themes

    #[serde(default)]
    dark_theme_colors: Vec<i32>,
}

impl RObject for AccentColor {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl AccentColor {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AccentColorBuilder {
        let mut inner = AccentColor::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AccentColorBuilder { inner }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn built_in_accent_color_id(&self) -> i32 {
        self.built_in_accent_color_id
    }

    pub fn light_theme_colors(&self) -> &Vec<i32> {
        &self.light_theme_colors
    }

    pub fn dark_theme_colors(&self) -> &Vec<i32> {
        &self.dark_theme_colors
    }
}

#[doc(hidden)]
pub struct AccentColorBuilder {
    inner: AccentColor,
}

#[deprecated]
pub type RTDAccentColorBuilder = AccentColorBuilder;

impl AccentColorBuilder {
    pub fn build(&self) -> AccentColor {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i32) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn built_in_accent_color_id(&mut self, built_in_accent_color_id: i32) -> &mut Self {
        self.inner.built_in_accent_color_id = built_in_accent_color_id;
        self
    }

    pub fn light_theme_colors(&mut self, light_theme_colors: Vec<i32>) -> &mut Self {
        self.inner.light_theme_colors = light_theme_colors;
        self
    }

    pub fn dark_theme_colors(&mut self, dark_theme_colors: Vec<i32>) -> &mut Self {
        self.inner.dark_theme_colors = dark_theme_colors;
        self
    }
}

impl AsRef<AccentColor> for AccentColor {
    fn as_ref(&self) -> &AccentColor {
        self
    }
}

impl AsRef<AccentColor> for AccentColorBuilder {
    fn as_ref(&self) -> &AccentColor {
        &self.inner
    }
}
