use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains information about background to set
pub trait TDInputBackground: Debug + RObject {}

/// Contains information about background to set
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputBackground {
    #[doc(hidden)]
    _Default(()),
    /// A background from a local file
    Local(InputBackgroundLocal),
    /// A background from the server
    Remote(InputBackgroundRemote),
}

impl Default for InputBackground {
    fn default() -> Self {
        InputBackground::_Default(())
    }
}

impl<'de> Deserialize<'de> for InputBackground {
    fn deserialize<D>(deserializer: D) -> Result<InputBackground, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          InputBackground,
          (inputBackgroundLocal, Local);
          (inputBackgroundRemote, Remote);

        )(deserializer)
    }
}

impl RObject for InputBackground {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            InputBackground::Local(t) => t.td_name(),
            InputBackground::Remote(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            InputBackground::Local(t) => t.extra(),
            InputBackground::Remote(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl InputBackground {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, InputBackground::_Default(_))
    }
}

impl AsRef<InputBackground> for InputBackground {
    fn as_ref(&self) -> &InputBackground {
        self
    }
}

/// A background from a local file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputBackgroundLocal {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Background file to use. Only inputFileLocal and inputFileGenerated are supported. The file must be in JPEG format for wallpapers and in PNG format for patterns
    background: InputFile,
}

impl RObject for InputBackgroundLocal {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "inputBackgroundLocal"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDInputBackground for InputBackgroundLocal {}

impl InputBackgroundLocal {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputBackgroundLocalBuilder {
        let mut inner = InputBackgroundLocal::default();
        inner.td_name = "inputBackgroundLocal".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDInputBackgroundLocalBuilder { inner }
    }

    pub fn background(&self) -> &InputFile {
        &self.background
    }
}

#[doc(hidden)]
pub struct RTDInputBackgroundLocalBuilder {
    inner: InputBackgroundLocal,
}

impl RTDInputBackgroundLocalBuilder {
    pub fn build(&self) -> InputBackgroundLocal {
        self.inner.clone()
    }

    pub fn background<T: AsRef<InputFile>>(&mut self, background: T) -> &mut Self {
        self.inner.background = background.as_ref().clone();
        self
    }
}

impl AsRef<InputBackgroundLocal> for InputBackgroundLocal {
    fn as_ref(&self) -> &InputBackgroundLocal {
        self
    }
}

impl AsRef<InputBackgroundLocal> for RTDInputBackgroundLocalBuilder {
    fn as_ref(&self) -> &InputBackgroundLocal {
        &self.inner
    }
}

/// A background from the server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputBackgroundRemote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// The background identifier
    #[serde(deserialize_with = "super::_common::number_from_string")]
    background_id: i64,
}

impl RObject for InputBackgroundRemote {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "inputBackgroundRemote"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDInputBackground for InputBackgroundRemote {}

impl InputBackgroundRemote {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputBackgroundRemoteBuilder {
        let mut inner = InputBackgroundRemote::default();
        inner.td_name = "inputBackgroundRemote".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDInputBackgroundRemoteBuilder { inner }
    }

    pub fn background_id(&self) -> i64 {
        self.background_id
    }
}

#[doc(hidden)]
pub struct RTDInputBackgroundRemoteBuilder {
    inner: InputBackgroundRemote,
}

impl RTDInputBackgroundRemoteBuilder {
    pub fn build(&self) -> InputBackgroundRemote {
        self.inner.clone()
    }

    pub fn background_id(&mut self, background_id: i64) -> &mut Self {
        self.inner.background_id = background_id;
        self
    }
}

impl AsRef<InputBackgroundRemote> for InputBackgroundRemote {
    fn as_ref(&self) -> &InputBackgroundRemote {
        self
    }
}

impl AsRef<InputBackgroundRemote> for RTDInputBackgroundRemoteBuilder {
    fn as_ref(&self) -> &InputBackgroundRemote {
        &self.inner
    }
}
