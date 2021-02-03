use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a keyboard button type
pub trait TDKeyboardButtonType: Debug + RObject {}

/// Describes a keyboard button type
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum KeyboardButtonType {
    #[doc(hidden)]
    _Default(()),
    /// A button that sends the user's location when pressed; available only in private chats
    RequestLocation(KeyboardButtonTypeRequestLocation),
    /// A button that sends the user's phone number when pressed; available only in private chats
    RequestPhoneNumber(KeyboardButtonTypeRequestPhoneNumber),
    /// A button that allows the user to create and send a poll when pressed; available only in private chats
    RequestPoll(KeyboardButtonTypeRequestPoll),
    /// A simple button, with text that should be sent when the button is pressed
    Text(KeyboardButtonTypeText),
}

impl Default for KeyboardButtonType {
    fn default() -> Self {
        KeyboardButtonType::_Default(())
    }
}

impl<'de> Deserialize<'de> for KeyboardButtonType {
    fn deserialize<D>(deserializer: D) -> Result<KeyboardButtonType, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          KeyboardButtonType,
          (keyboardButtonTypeRequestLocation, RequestLocation);
          (keyboardButtonTypeRequestPhoneNumber, RequestPhoneNumber);
          (keyboardButtonTypeRequestPoll, RequestPoll);
          (keyboardButtonTypeText, Text);

        )(deserializer)
    }
}

impl RObject for KeyboardButtonType {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            KeyboardButtonType::RequestLocation(t) => t.td_name(),
            KeyboardButtonType::RequestPhoneNumber(t) => t.td_name(),
            KeyboardButtonType::RequestPoll(t) => t.td_name(),
            KeyboardButtonType::Text(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            KeyboardButtonType::RequestLocation(t) => t.extra(),
            KeyboardButtonType::RequestPhoneNumber(t) => t.extra(),
            KeyboardButtonType::RequestPoll(t) => t.extra(),
            KeyboardButtonType::Text(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            KeyboardButtonType::RequestLocation(t) => t.client_id(),
            KeyboardButtonType::RequestPhoneNumber(t) => t.client_id(),
            KeyboardButtonType::RequestPoll(t) => t.client_id(),
            KeyboardButtonType::Text(t) => t.client_id(),

            _ => None,
        }
    }
}

impl KeyboardButtonType {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, KeyboardButtonType::_Default(_))
    }
}

impl AsRef<KeyboardButtonType> for KeyboardButtonType {
    fn as_ref(&self) -> &KeyboardButtonType {
        self
    }
}

/// A button that sends the user's location when pressed; available only in private chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButtonTypeRequestLocation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for KeyboardButtonTypeRequestLocation {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "keyboardButtonTypeRequestLocation"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDKeyboardButtonType for KeyboardButtonTypeRequestLocation {}

impl KeyboardButtonTypeRequestLocation {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDKeyboardButtonTypeRequestLocationBuilder {
        let mut inner = KeyboardButtonTypeRequestLocation::default();
        inner.td_name = "keyboardButtonTypeRequestLocation".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDKeyboardButtonTypeRequestLocationBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDKeyboardButtonTypeRequestLocationBuilder {
    inner: KeyboardButtonTypeRequestLocation,
}

impl RTDKeyboardButtonTypeRequestLocationBuilder {
    pub fn build(&self) -> KeyboardButtonTypeRequestLocation {
        self.inner.clone()
    }
}

impl AsRef<KeyboardButtonTypeRequestLocation> for KeyboardButtonTypeRequestLocation {
    fn as_ref(&self) -> &KeyboardButtonTypeRequestLocation {
        self
    }
}

impl AsRef<KeyboardButtonTypeRequestLocation> for RTDKeyboardButtonTypeRequestLocationBuilder {
    fn as_ref(&self) -> &KeyboardButtonTypeRequestLocation {
        &self.inner
    }
}

/// A button that sends the user's phone number when pressed; available only in private chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButtonTypeRequestPhoneNumber {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for KeyboardButtonTypeRequestPhoneNumber {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "keyboardButtonTypeRequestPhoneNumber"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDKeyboardButtonType for KeyboardButtonTypeRequestPhoneNumber {}

impl KeyboardButtonTypeRequestPhoneNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDKeyboardButtonTypeRequestPhoneNumberBuilder {
        let mut inner = KeyboardButtonTypeRequestPhoneNumber::default();
        inner.td_name = "keyboardButtonTypeRequestPhoneNumber".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDKeyboardButtonTypeRequestPhoneNumberBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDKeyboardButtonTypeRequestPhoneNumberBuilder {
    inner: KeyboardButtonTypeRequestPhoneNumber,
}

impl RTDKeyboardButtonTypeRequestPhoneNumberBuilder {
    pub fn build(&self) -> KeyboardButtonTypeRequestPhoneNumber {
        self.inner.clone()
    }
}

impl AsRef<KeyboardButtonTypeRequestPhoneNumber> for KeyboardButtonTypeRequestPhoneNumber {
    fn as_ref(&self) -> &KeyboardButtonTypeRequestPhoneNumber {
        self
    }
}

impl AsRef<KeyboardButtonTypeRequestPhoneNumber>
    for RTDKeyboardButtonTypeRequestPhoneNumberBuilder
{
    fn as_ref(&self) -> &KeyboardButtonTypeRequestPhoneNumber {
        &self.inner
    }
}

/// A button that allows the user to create and send a poll when pressed; available only in private chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButtonTypeRequestPoll {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// If true, only regular polls must be allowed to create
    force_regular: bool,
    /// If true, only polls in quiz mode must be allowed to create
    force_quiz: bool,
}

impl RObject for KeyboardButtonTypeRequestPoll {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "keyboardButtonTypeRequestPoll"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDKeyboardButtonType for KeyboardButtonTypeRequestPoll {}

impl KeyboardButtonTypeRequestPoll {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDKeyboardButtonTypeRequestPollBuilder {
        let mut inner = KeyboardButtonTypeRequestPoll::default();
        inner.td_name = "keyboardButtonTypeRequestPoll".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDKeyboardButtonTypeRequestPollBuilder { inner }
    }

    pub fn force_regular(&self) -> bool {
        self.force_regular
    }

    pub fn force_quiz(&self) -> bool {
        self.force_quiz
    }
}

#[doc(hidden)]
pub struct RTDKeyboardButtonTypeRequestPollBuilder {
    inner: KeyboardButtonTypeRequestPoll,
}

impl RTDKeyboardButtonTypeRequestPollBuilder {
    pub fn build(&self) -> KeyboardButtonTypeRequestPoll {
        self.inner.clone()
    }

    pub fn force_regular(&mut self, force_regular: bool) -> &mut Self {
        self.inner.force_regular = force_regular;
        self
    }

    pub fn force_quiz(&mut self, force_quiz: bool) -> &mut Self {
        self.inner.force_quiz = force_quiz;
        self
    }
}

impl AsRef<KeyboardButtonTypeRequestPoll> for KeyboardButtonTypeRequestPoll {
    fn as_ref(&self) -> &KeyboardButtonTypeRequestPoll {
        self
    }
}

impl AsRef<KeyboardButtonTypeRequestPoll> for RTDKeyboardButtonTypeRequestPollBuilder {
    fn as_ref(&self) -> &KeyboardButtonTypeRequestPoll {
        &self.inner
    }
}

/// A simple button, with text that should be sent when the button is pressed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyboardButtonTypeText {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for KeyboardButtonTypeText {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "keyboardButtonTypeText"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDKeyboardButtonType for KeyboardButtonTypeText {}

impl KeyboardButtonTypeText {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDKeyboardButtonTypeTextBuilder {
        let mut inner = KeyboardButtonTypeText::default();
        inner.td_name = "keyboardButtonTypeText".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDKeyboardButtonTypeTextBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDKeyboardButtonTypeTextBuilder {
    inner: KeyboardButtonTypeText,
}

impl RTDKeyboardButtonTypeTextBuilder {
    pub fn build(&self) -> KeyboardButtonTypeText {
        self.inner.clone()
    }
}

impl AsRef<KeyboardButtonTypeText> for KeyboardButtonTypeText {
    fn as_ref(&self) -> &KeyboardButtonTypeText {
        self
    }
}

impl AsRef<KeyboardButtonTypeText> for RTDKeyboardButtonTypeTextBuilder {
    fn as_ref(&self) -> &KeyboardButtonTypeText {
        &self.inner
    }
}
