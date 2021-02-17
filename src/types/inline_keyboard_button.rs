use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a single button in an inline keyboard
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineKeyboardButton {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text of the button
    text: String,
    /// Type of the button

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "InlineKeyboardButtonType::_is_default")]
    type_: InlineKeyboardButtonType,
}

impl RObject for InlineKeyboardButton {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl InlineKeyboardButton {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInlineKeyboardButtonBuilder {
        let mut inner = InlineKeyboardButton::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInlineKeyboardButtonBuilder { inner }
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn type_(&self) -> &InlineKeyboardButtonType {
        &self.type_
    }
}

#[doc(hidden)]
pub struct RTDInlineKeyboardButtonBuilder {
    inner: InlineKeyboardButton,
}

impl RTDInlineKeyboardButtonBuilder {
    pub fn build(&self) -> InlineKeyboardButton {
        self.inner.clone()
    }

    pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().to_string();
        self
    }

    pub fn type_<T: AsRef<InlineKeyboardButtonType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }
}

impl AsRef<InlineKeyboardButton> for InlineKeyboardButton {
    fn as_ref(&self) -> &InlineKeyboardButton {
        self
    }
}

impl AsRef<InlineKeyboardButton> for RTDInlineKeyboardButtonBuilder {
    fn as_ref(&self) -> &InlineKeyboardButton {
        &self.inner
    }
}
