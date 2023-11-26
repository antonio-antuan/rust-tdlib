use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes accent color and background custom emoji for the current user; for Telegram Premium users only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetAccentColor {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the accent color to use

    #[serde(default)]
    accent_color_id: i32,
    /// Identifier of a custom emoji to be shown on the reply header background; 0 if none

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    background_custom_emoji_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetAccentColor {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetAccentColor {}

impl SetAccentColor {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetAccentColorBuilder {
        let mut inner = SetAccentColor::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setAccentColor".to_string();

        SetAccentColorBuilder { inner }
    }

    pub fn accent_color_id(&self) -> i32 {
        self.accent_color_id
    }

    pub fn background_custom_emoji_id(&self) -> i64 {
        self.background_custom_emoji_id
    }
}

#[doc(hidden)]
pub struct SetAccentColorBuilder {
    inner: SetAccentColor,
}

#[deprecated]
pub type RTDSetAccentColorBuilder = SetAccentColorBuilder;

impl SetAccentColorBuilder {
    pub fn build(&self) -> SetAccentColor {
        self.inner.clone()
    }

    pub fn accent_color_id(&mut self, accent_color_id: i32) -> &mut Self {
        self.inner.accent_color_id = accent_color_id;
        self
    }

    pub fn background_custom_emoji_id(&mut self, background_custom_emoji_id: i64) -> &mut Self {
        self.inner.background_custom_emoji_id = background_custom_emoji_id;
        self
    }
}

impl AsRef<SetAccentColor> for SetAccentColor {
    fn as_ref(&self) -> &SetAccentColor {
        self
    }
}

impl AsRef<SetAccentColor> for SetAccentColorBuilder {
    fn as_ref(&self) -> &SetAccentColor {
        &self.inner
    }
}
