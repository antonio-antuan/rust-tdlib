use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a color to highlight a bot added to attachment menu
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttachmentMenuBotColor {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Color in the RGB24 format for light themes
    light_color: i32,
    /// Color in the RGB24 format for dark themes
    dark_color: i32,
}

impl RObject for AttachmentMenuBotColor {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl AttachmentMenuBotColor {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDAttachmentMenuBotColorBuilder {
        let mut inner = AttachmentMenuBotColor::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDAttachmentMenuBotColorBuilder { inner }
    }

    pub fn light_color(&self) -> i32 {
        self.light_color
    }

    pub fn dark_color(&self) -> i32 {
        self.dark_color
    }
}

#[doc(hidden)]
pub struct RTDAttachmentMenuBotColorBuilder {
    inner: AttachmentMenuBotColor,
}

impl RTDAttachmentMenuBotColorBuilder {
    pub fn build(&self) -> AttachmentMenuBotColor {
        self.inner.clone()
    }

    pub fn light_color(&mut self, light_color: i32) -> &mut Self {
        self.inner.light_color = light_color;
        self
    }

    pub fn dark_color(&mut self, dark_color: i32) -> &mut Self {
        self.inner.dark_color = dark_color;
        self
    }
}

impl AsRef<AttachmentMenuBotColor> for AttachmentMenuBotColor {
    fn as_ref(&self) -> &AttachmentMenuBotColor {
        self
    }
}

impl AsRef<AttachmentMenuBotColor> for RTDAttachmentMenuBotColorBuilder {
    fn as_ref(&self) -> &AttachmentMenuBotColor {
        &self.inner
    }
}
