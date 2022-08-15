use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains information about background to set
pub trait TDInputBackground: Debug + RObject {}

/// Contains information about background to set
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputBackground {
    #[doc(hidden)]
    _Default,
    /// A background from a local file
    #[serde(rename = "inputBackgroundLocal")]
    Local(InputBackgroundLocal),
    /// A background from the server
    #[serde(rename = "inputBackgroundRemote")]
    Remote(InputBackgroundRemote),
}

impl Default for InputBackground {
    fn default() -> Self {
        InputBackground::_Default
    }
}

impl RObject for InputBackground {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            InputBackground::Local(t) => t.extra(),
            InputBackground::Remote(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            InputBackground::Local(t) => t.client_id(),
            InputBackground::Remote(t) => t.client_id(),

            _ => None,
        }
    }
}

impl InputBackground {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, InputBackground::_Default)
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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Background file to use. Only inputFileLocal and inputFileGenerated are supported. The file must be in JPEG format for wallpapers and in PNG format for patterns

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    background: InputFile,
}

impl RObject for InputBackgroundLocal {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputBackground for InputBackgroundLocal {}

impl InputBackgroundLocal {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputBackgroundLocalBuilder {
        let mut inner = InputBackgroundLocal::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputBackgroundLocalBuilder { inner }
    }

    pub fn background(&self) -> &InputFile {
        &self.background
    }
}

#[doc(hidden)]
pub struct InputBackgroundLocalBuilder {
    inner: InputBackgroundLocal,
}

#[deprecated]
pub type RTDInputBackgroundLocalBuilder = InputBackgroundLocalBuilder;

impl InputBackgroundLocalBuilder {
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

impl AsRef<InputBackgroundLocal> for InputBackgroundLocalBuilder {
    fn as_ref(&self) -> &InputBackgroundLocal {
        &self.inner
    }
}

/// A background from the server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputBackgroundRemote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The background identifier

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    background_id: i64,
}

impl RObject for InputBackgroundRemote {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputBackground for InputBackgroundRemote {}

impl InputBackgroundRemote {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputBackgroundRemoteBuilder {
        let mut inner = InputBackgroundRemote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputBackgroundRemoteBuilder { inner }
    }

    pub fn background_id(&self) -> i64 {
        self.background_id
    }
}

#[doc(hidden)]
pub struct InputBackgroundRemoteBuilder {
    inner: InputBackgroundRemote,
}

#[deprecated]
pub type RTDInputBackgroundRemoteBuilder = InputBackgroundRemoteBuilder;

impl InputBackgroundRemoteBuilder {
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

impl AsRef<InputBackgroundRemote> for InputBackgroundRemoteBuilder {
    fn as_ref(&self) -> &InputBackgroundRemote {
        &self.inner
    }
}
