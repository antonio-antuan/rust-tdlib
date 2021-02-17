use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a single button in a bot keyboard
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButton {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text of the button
    text: String,
    /// Type of the button

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "KeyboardButtonType::_is_default")]
    type_: KeyboardButtonType,
}

impl RObject for KeyboardButton {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl KeyboardButton {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDKeyboardButtonBuilder {
        let mut inner = KeyboardButton::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDKeyboardButtonBuilder { inner }
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn type_(&self) -> &KeyboardButtonType {
        &self.type_
    }
}

#[doc(hidden)]
pub struct RTDKeyboardButtonBuilder {
    inner: KeyboardButton,
}

impl RTDKeyboardButtonBuilder {
    pub fn build(&self) -> KeyboardButton {
        self.inner.clone()
    }

    pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().to_string();
        self
    }

    pub fn type_<T: AsRef<KeyboardButtonType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }
}

impl AsRef<KeyboardButton> for KeyboardButton {
    fn as_ref(&self) -> &KeyboardButton {
        self
    }
}

impl AsRef<KeyboardButton> for RTDKeyboardButtonBuilder {
    fn as_ref(&self) -> &KeyboardButton {
        &self.inner
    }
}
