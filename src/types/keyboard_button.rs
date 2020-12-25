
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Represents a single button in a bot keyboard
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButton {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text of the button
  text: String,
  /// Type of the button
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: KeyboardButtonType,
  
}

impl RObject for KeyboardButton {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "keyboardButton" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl KeyboardButton {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDKeyboardButtonBuilder {
    let mut inner = KeyboardButton::default();
    inner.td_name = "keyboardButton".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDKeyboardButtonBuilder { inner }
  }

  pub fn text(&self) -> &String { &self.text }

  pub fn type_(&self) -> &KeyboardButtonType { &self.type_ }

}

#[doc(hidden)]
pub struct RTDKeyboardButtonBuilder {
  inner: KeyboardButton
}

impl RTDKeyboardButtonBuilder {
  pub fn build(&self) -> KeyboardButton { self.inner.clone() }

   
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
  fn as_ref(&self) -> &KeyboardButton { self }
}

impl AsRef<KeyboardButton> for RTDKeyboardButtonBuilder {
  fn as_ref(&self) -> &KeyboardButton { &self.inner }
}



