use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Represents a payload of a callback query
pub trait TDCallbackQueryPayload: Debug + RObject {}

/// Represents a payload of a callback query
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CallbackQueryPayload {
    #[doc(hidden)]
    _Default(()),
    /// The payload for a general callback button
    Data(CallbackQueryPayloadData),
    /// The payload for a callback button requiring password
    DataWithPassword(CallbackQueryPayloadDataWithPassword),
    /// The payload for a game callback button
    Game(CallbackQueryPayloadGame),
}

impl Default for CallbackQueryPayload {
    fn default() -> Self {
        CallbackQueryPayload::_Default(())
    }
}

impl<'de> Deserialize<'de> for CallbackQueryPayload {
    fn deserialize<D>(deserializer: D) -> Result<CallbackQueryPayload, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          CallbackQueryPayload,
          (callbackQueryPayloadData, Data);
          (callbackQueryPayloadDataWithPassword, DataWithPassword);
          (callbackQueryPayloadGame, Game);

        )(deserializer)
    }
}

impl RObject for CallbackQueryPayload {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            CallbackQueryPayload::Data(t) => t.td_name(),
            CallbackQueryPayload::DataWithPassword(t) => t.td_name(),
            CallbackQueryPayload::Game(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            CallbackQueryPayload::Data(t) => t.extra(),
            CallbackQueryPayload::DataWithPassword(t) => t.extra(),
            CallbackQueryPayload::Game(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
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
        matches!(self, CallbackQueryPayload::_Default(_))
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
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
    fn td_name(&self) -> &'static str {
        "callbackQueryPayloadData"
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

impl TDCallbackQueryPayload for CallbackQueryPayloadData {}

impl CallbackQueryPayloadData {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallbackQueryPayloadDataBuilder {
        let mut inner = CallbackQueryPayloadData::default();
        inner.td_name = "callbackQueryPayloadData".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
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
    fn td_name(&self) -> &'static str {
        "callbackQueryPayloadDataWithPassword"
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

impl TDCallbackQueryPayload for CallbackQueryPayloadDataWithPassword {}

impl CallbackQueryPayloadDataWithPassword {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallbackQueryPayloadDataWithPasswordBuilder {
        let mut inner = CallbackQueryPayloadDataWithPassword::default();
        inner.td_name = "callbackQueryPayloadDataWithPassword".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
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
    fn td_name(&self) -> &'static str {
        "callbackQueryPayloadGame"
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

impl TDCallbackQueryPayload for CallbackQueryPayloadGame {}

impl CallbackQueryPayloadGame {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallbackQueryPayloadGameBuilder {
        let mut inner = CallbackQueryPayloadGame::default();
        inner.td_name = "callbackQueryPayloadGame".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
