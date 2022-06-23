use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains parameters of the application theme
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThemeParameters {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A color of the background in the RGB24 format
    background_color: i32,
    /// A secondary color for the background in the RGB24 format
    secondary_background_color: i32,
    /// A color of text in the RGB24 format
    text_color: i32,
    /// A color of hints in the RGB24 format
    hint_color: i32,
    /// A color of links in the RGB24 format
    link_color: i32,
    /// A color of the buttons in the RGB24 format
    button_color: i32,
    /// A color of text on the buttons in the RGB24 format
    button_text_color: i32,
}

impl RObject for ThemeParameters {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ThemeParameters {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDThemeParametersBuilder {
        let mut inner = ThemeParameters::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDThemeParametersBuilder { inner }
    }

    pub fn background_color(&self) -> i32 {
        self.background_color
    }

    pub fn secondary_background_color(&self) -> i32 {
        self.secondary_background_color
    }

    pub fn text_color(&self) -> i32 {
        self.text_color
    }

    pub fn hint_color(&self) -> i32 {
        self.hint_color
    }

    pub fn link_color(&self) -> i32 {
        self.link_color
    }

    pub fn button_color(&self) -> i32 {
        self.button_color
    }

    pub fn button_text_color(&self) -> i32 {
        self.button_text_color
    }
}

#[doc(hidden)]
pub struct RTDThemeParametersBuilder {
    inner: ThemeParameters,
}

impl RTDThemeParametersBuilder {
    pub fn build(&self) -> ThemeParameters {
        self.inner.clone()
    }

    pub fn background_color(&mut self, background_color: i32) -> &mut Self {
        self.inner.background_color = background_color;
        self
    }

    pub fn secondary_background_color(&mut self, secondary_background_color: i32) -> &mut Self {
        self.inner.secondary_background_color = secondary_background_color;
        self
    }

    pub fn text_color(&mut self, text_color: i32) -> &mut Self {
        self.inner.text_color = text_color;
        self
    }

    pub fn hint_color(&mut self, hint_color: i32) -> &mut Self {
        self.inner.hint_color = hint_color;
        self
    }

    pub fn link_color(&mut self, link_color: i32) -> &mut Self {
        self.inner.link_color = link_color;
        self
    }

    pub fn button_color(&mut self, button_color: i32) -> &mut Self {
        self.inner.button_color = button_color;
        self
    }

    pub fn button_text_color(&mut self, button_text_color: i32) -> &mut Self {
        self.inner.button_text_color = button_text_color;
        self
    }
}

impl AsRef<ThemeParameters> for ThemeParameters {
    fn as_ref(&self) -> &ThemeParameters {
        self
    }
}

impl AsRef<ThemeParameters> for RTDThemeParametersBuilder {
    fn as_ref(&self) -> &ThemeParameters {
        &self.inner
    }
}
