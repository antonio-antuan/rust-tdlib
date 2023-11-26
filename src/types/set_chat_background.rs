use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the background in a specific chat. Supported only in private and secret chats with non-deleted users
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetChatBackground {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// The input background to use; pass null to create a new filled background or to remove the current background

    #[serde(skip_serializing_if = "InputBackground::_is_default")]
    background: InputBackground,
    /// Background type; pass null to remove the current background

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "BackgroundType::_is_default")]
    type_: BackgroundType,
    /// Dimming of the background in dark themes, as a percentage; 0-100

    #[serde(default)]
    dark_theme_dimming: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetChatBackground {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetChatBackground {}

impl SetChatBackground {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetChatBackgroundBuilder {
        let mut inner = SetChatBackground::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setChatBackground".to_string();

        SetChatBackgroundBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn background(&self) -> &InputBackground {
        &self.background
    }

    pub fn type_(&self) -> &BackgroundType {
        &self.type_
    }

    pub fn dark_theme_dimming(&self) -> i32 {
        self.dark_theme_dimming
    }
}

#[doc(hidden)]
pub struct SetChatBackgroundBuilder {
    inner: SetChatBackground,
}

#[deprecated]
pub type RTDSetChatBackgroundBuilder = SetChatBackgroundBuilder;

impl SetChatBackgroundBuilder {
    pub fn build(&self) -> SetChatBackground {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn background<T: AsRef<InputBackground>>(&mut self, background: T) -> &mut Self {
        self.inner.background = background.as_ref().clone();
        self
    }

    pub fn type_<T: AsRef<BackgroundType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }

    pub fn dark_theme_dimming(&mut self, dark_theme_dimming: i32) -> &mut Self {
        self.inner.dark_theme_dimming = dark_theme_dimming;
        self
    }
}

impl AsRef<SetChatBackground> for SetChatBackground {
    fn as_ref(&self) -> &SetChatBackground {
        self
    }
}

impl AsRef<SetChatBackground> for SetChatBackgroundBuilder {
    fn as_ref(&self) -> &SetChatBackground {
        &self.inner
    }
}
