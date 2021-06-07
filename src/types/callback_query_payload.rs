use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Represents a payload of a callback query
pub trait TDCallbackQueryPayload: Debug + RObject {}

/// Represents a payload of a callback query
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CallbackQueryPayload {
    #[doc(hidden)]
    _Default,
    /// The payload for a general callback button
    #[serde(rename(
        serialize = "callbackQueryPayloadData",
        deserialize = "callbackQueryPayloadData"
    ))]
    Data(CallbackQueryPayloadData),
    /// The payload for a callback button requiring password
    #[serde(rename(
        serialize = "callbackQueryPayloadDataWithPassword",
        deserialize = "callbackQueryPayloadDataWithPassword"
    ))]
    DataWithPassword(CallbackQueryPayloadDataWithPassword),
    /// The payload for a game callback button
    #[serde(rename(
        serialize = "callbackQueryPayloadGame",
        deserialize = "callbackQueryPayloadGame"
    ))]
    Game(CallbackQueryPayloadGame),
}

impl Default for CallbackQueryPayload {
    fn default() -> Self {
        CallbackQueryPayload::_Default
    }
}

impl RObject for CallbackQueryPayload {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            CallbackQueryPayload::Data(t) => t.extra(),
            CallbackQueryPayload::DataWithPassword(t) => t.extra(),
            CallbackQueryPayload::Game(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            CallbackQueryPayload::Data(t) => t.client_id(),
            CallbackQueryPayload::DataWithPassword(t) => t.client_id(),
            CallbackQueryPayload::Game(t) => t.client_id(),

            _ => None,
        }
    }
}

impl CallbackQueryPayload {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, CallbackQueryPayload::_Default)
    }
}

impl AsRef<CallbackQueryPayload> for CallbackQueryPayload {
    fn as_ref(&self) -> &CallbackQueryPayload {
        self
    }
}

/// The payload for a general callback button
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallbackQueryPayloadData {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Data that was attached to the callback button
    data: String,
}

impl RObject for CallbackQueryPayloadData {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCallbackQueryPayload for CallbackQueryPayloadData {}

impl CallbackQueryPayloadData {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallbackQueryPayloadDataBuilder {
        let mut inner = CallbackQueryPayloadData::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDCallbackQueryPayloadDataBuilder { inner }
    }

    pub fn data(&self) -> &String {
        &self.data
    }
}

#[doc(hidden)]
pub struct RTDCallbackQueryPayloadDataBuilder {
    inner: CallbackQueryPayloadData,
}

impl RTDCallbackQueryPayloadDataBuilder {
    pub fn build(&self) -> CallbackQueryPayloadData {
        self.inner.clone()
    }

    pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
        self.inner.data = data.as_ref().to_string();
        self
    }
}

impl AsRef<CallbackQueryPayloadData> for CallbackQueryPayloadData {
    fn as_ref(&self) -> &CallbackQueryPayloadData {
        self
    }
}

impl AsRef<CallbackQueryPayloadData> for RTDCallbackQueryPayloadDataBuilder {
    fn as_ref(&self) -> &CallbackQueryPayloadData {
        &self.inner
    }
}

/// The payload for a callback button requiring password
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallbackQueryPayloadDataWithPassword {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The password for the current user
    password: String,
    /// Data that was attached to the callback button
    data: String,
}

impl RObject for CallbackQueryPayloadDataWithPassword {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCallbackQueryPayload for CallbackQueryPayloadDataWithPassword {}

impl CallbackQueryPayloadDataWithPassword {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallbackQueryPayloadDataWithPasswordBuilder {
        let mut inner = CallbackQueryPayloadDataWithPassword::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDCallbackQueryPayloadDataWithPasswordBuilder { inner }
    }

    pub fn password(&self) -> &String {
        &self.password
    }

    pub fn data(&self) -> &String {
        &self.data
    }
}

#[doc(hidden)]
pub struct RTDCallbackQueryPayloadDataWithPasswordBuilder {
    inner: CallbackQueryPayloadDataWithPassword,
}

impl RTDCallbackQueryPayloadDataWithPasswordBuilder {
    pub fn build(&self) -> CallbackQueryPayloadDataWithPassword {
        self.inner.clone()
    }

    pub fn password<T: AsRef<str>>(&mut self, password: T) -> &mut Self {
        self.inner.password = password.as_ref().to_string();
        self
    }

    pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
        self.inner.data = data.as_ref().to_string();
        self
    }
}

impl AsRef<CallbackQueryPayloadDataWithPassword> for CallbackQueryPayloadDataWithPassword {
    fn as_ref(&self) -> &CallbackQueryPayloadDataWithPassword {
        self
    }
}

impl AsRef<CallbackQueryPayloadDataWithPassword>
    for RTDCallbackQueryPayloadDataWithPasswordBuilder
{
    fn as_ref(&self) -> &CallbackQueryPayloadDataWithPassword {
        &self.inner
    }
}

/// The payload for a game callback button
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallbackQueryPayloadGame {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A short name of the game that was attached to the callback button
    game_short_name: String,
}

impl RObject for CallbackQueryPayloadGame {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCallbackQueryPayload for CallbackQueryPayloadGame {}

impl CallbackQueryPayloadGame {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallbackQueryPayloadGameBuilder {
        let mut inner = CallbackQueryPayloadGame::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDCallbackQueryPayloadGameBuilder { inner }
    }

    pub fn game_short_name(&self) -> &String {
        &self.game_short_name
    }
}

#[doc(hidden)]
pub struct RTDCallbackQueryPayloadGameBuilder {
    inner: CallbackQueryPayloadGame,
}

impl RTDCallbackQueryPayloadGameBuilder {
    pub fn build(&self) -> CallbackQueryPayloadGame {
        self.inner.clone()
    }

    pub fn game_short_name<T: AsRef<str>>(&mut self, game_short_name: T) -> &mut Self {
        self.inner.game_short_name = game_short_name.as_ref().to_string();
        self
    }
}

impl AsRef<CallbackQueryPayloadGame> for CallbackQueryPayloadGame {
    fn as_ref(&self) -> &CallbackQueryPayloadGame {
        self
    }
}

impl AsRef<CallbackQueryPayloadGame> for RTDCallbackQueryPayloadGameBuilder {
    fn as_ref(&self) -> &CallbackQueryPayloadGame {
        &self.inner
    }
}
