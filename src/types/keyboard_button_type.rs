use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes a keyboard button type
pub trait TDKeyboardButtonType: Debug + RObject {}

/// Describes a keyboard button type
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum KeyboardButtonType {
    #[doc(hidden)]
    _Default,
    /// A button that sends the user's location when pressed; available only in private chats
    #[serde(rename(
        serialize = "keyboardButtonTypeRequestLocation",
        deserialize = "keyboardButtonTypeRequestLocation"
    ))]
    RequestLocation(KeyboardButtonTypeRequestLocation),
    /// A button that sends the user's phone number when pressed; available only in private chats
    #[serde(rename(
        serialize = "keyboardButtonTypeRequestPhoneNumber",
        deserialize = "keyboardButtonTypeRequestPhoneNumber"
    ))]
    RequestPhoneNumber(KeyboardButtonTypeRequestPhoneNumber),
    /// A button that allows the user to create and send a poll when pressed; available only in private chats
    #[serde(rename(
        serialize = "keyboardButtonTypeRequestPoll",
        deserialize = "keyboardButtonTypeRequestPoll"
    ))]
    RequestPoll(KeyboardButtonTypeRequestPoll),
    /// A simple button, with text that should be sent when the button is pressed
    #[serde(rename(
        serialize = "keyboardButtonTypeText",
        deserialize = "keyboardButtonTypeText"
    ))]
    Text(KeyboardButtonTypeText),
}

impl Default for KeyboardButtonType {
    fn default() -> Self {
        KeyboardButtonType::_Default
    }
}

impl RObject for KeyboardButtonType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            KeyboardButtonType::RequestLocation(t) => t.extra(),
            KeyboardButtonType::RequestPhoneNumber(t) => t.extra(),
            KeyboardButtonType::RequestPoll(t) => t.extra(),
            KeyboardButtonType::Text(t) => t.extra(),

            _ => None,
        }
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
        matches!(self, KeyboardButtonType::_Default)
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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for KeyboardButtonTypeRequestLocation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDKeyboardButtonType for KeyboardButtonTypeRequestLocation {}

impl KeyboardButtonTypeRequestLocation {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDKeyboardButtonTypeRequestLocationBuilder {
        let mut inner = KeyboardButtonTypeRequestLocation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for KeyboardButtonTypeRequestPhoneNumber {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDKeyboardButtonType for KeyboardButtonTypeRequestPhoneNumber {}

impl KeyboardButtonTypeRequestPhoneNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDKeyboardButtonTypeRequestPhoneNumberBuilder {
        let mut inner = KeyboardButtonTypeRequestPhoneNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDKeyboardButtonType for KeyboardButtonTypeRequestPoll {}

impl KeyboardButtonTypeRequestPoll {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDKeyboardButtonTypeRequestPollBuilder {
        let mut inner = KeyboardButtonTypeRequestPoll::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for KeyboardButtonTypeText {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDKeyboardButtonType for KeyboardButtonTypeText {}

impl KeyboardButtonTypeText {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDKeyboardButtonTypeTextBuilder {
        let mut inner = KeyboardButtonTypeText::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
