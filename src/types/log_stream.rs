use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes a stream to which TDLib internal log is written
pub trait TDLogStream: Debug + RObject {}

/// Describes a stream to which TDLib internal log is written
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum LogStream {
    #[doc(hidden)]
    _Default,
    /// Returns information about currently used log stream for internal logging of TDLib. Can be called synchronously
    #[serde(rename(serialize = "getLogStream", deserialize = "getLogStream"))]
    GetLogStream(GetLogStream),
    /// The log is written to stderr or an OS specific log
    #[serde(rename(serialize = "logStreamDefault", deserialize = "logStreamDefault"))]
    Default(LogStreamDefault),
    /// The log is written nowhere
    #[serde(rename(serialize = "logStreamEmpty", deserialize = "logStreamEmpty"))]
    Empty(LogStreamEmpty),
    /// The log is written to a file
    #[serde(rename(serialize = "logStreamFile", deserialize = "logStreamFile"))]
    File(LogStreamFile),
}

impl Default for LogStream {
    fn default() -> Self {
        LogStream::_Default
    }
}

impl RObject for LogStream {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            LogStream::GetLogStream(t) => t.extra(),
            LogStream::Default(t) => t.extra(),
            LogStream::Empty(t) => t.extra(),
            LogStream::File(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            LogStream::GetLogStream(t) => t.client_id(),
            LogStream::Default(t) => t.client_id(),
            LogStream::Empty(t) => t.client_id(),
            LogStream::File(t) => t.client_id(),

            _ => None,
        }
    }
}

impl LogStream {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, LogStream::_Default)
    }
}

impl AsRef<LogStream> for LogStream {
    fn as_ref(&self) -> &LogStream {
        self
    }
}

/// The log is written to stderr or an OS specific log
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogStreamDefault {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for LogStreamDefault {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDLogStream for LogStreamDefault {}

impl LogStreamDefault {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDLogStreamDefaultBuilder {
        let mut inner = LogStreamDefault::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDLogStreamDefaultBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDLogStreamDefaultBuilder {
    inner: LogStreamDefault,
}

impl RTDLogStreamDefaultBuilder {
    pub fn build(&self) -> LogStreamDefault {
        self.inner.clone()
    }
}

impl AsRef<LogStreamDefault> for LogStreamDefault {
    fn as_ref(&self) -> &LogStreamDefault {
        self
    }
}

impl AsRef<LogStreamDefault> for RTDLogStreamDefaultBuilder {
    fn as_ref(&self) -> &LogStreamDefault {
        &self.inner
    }
}

/// The log is written nowhere
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogStreamEmpty {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for LogStreamEmpty {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDLogStream for LogStreamEmpty {}

impl LogStreamEmpty {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDLogStreamEmptyBuilder {
        let mut inner = LogStreamEmpty::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDLogStreamEmptyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDLogStreamEmptyBuilder {
    inner: LogStreamEmpty,
}

impl RTDLogStreamEmptyBuilder {
    pub fn build(&self) -> LogStreamEmpty {
        self.inner.clone()
    }
}

impl AsRef<LogStreamEmpty> for LogStreamEmpty {
    fn as_ref(&self) -> &LogStreamEmpty {
        self
    }
}

impl AsRef<LogStreamEmpty> for RTDLogStreamEmptyBuilder {
    fn as_ref(&self) -> &LogStreamEmpty {
        &self.inner
    }
}

/// The log is written to a file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogStreamFile {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Path to the file to where the internal TDLib log will be written
    path: String,
    /// The maximum size of the file to where the internal TDLib log is written before the file will be auto-rotated
    max_file_size: i64,
    /// Pass true to additionally redirect stderr to the log file. Ignored on Windows
    redirect_stderr: bool,
}

impl RObject for LogStreamFile {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDLogStream for LogStreamFile {}

impl LogStreamFile {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDLogStreamFileBuilder {
        let mut inner = LogStreamFile::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDLogStreamFileBuilder { inner }
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn max_file_size(&self) -> i64 {
        self.max_file_size
    }

    pub fn redirect_stderr(&self) -> bool {
        self.redirect_stderr
    }
}

#[doc(hidden)]
pub struct RTDLogStreamFileBuilder {
    inner: LogStreamFile,
}

impl RTDLogStreamFileBuilder {
    pub fn build(&self) -> LogStreamFile {
        self.inner.clone()
    }

    pub fn path<T: AsRef<str>>(&mut self, path: T) -> &mut Self {
        self.inner.path = path.as_ref().to_string();
        self
    }

    pub fn max_file_size(&mut self, max_file_size: i64) -> &mut Self {
        self.inner.max_file_size = max_file_size;
        self
    }

    pub fn redirect_stderr(&mut self, redirect_stderr: bool) -> &mut Self {
        self.inner.redirect_stderr = redirect_stderr;
        self
    }
}

impl AsRef<LogStreamFile> for LogStreamFile {
    fn as_ref(&self) -> &LogStreamFile {
        self
    }
}

impl AsRef<LogStreamFile> for RTDLogStreamFileBuilder {
    fn as_ref(&self) -> &LogStreamFile {
        &self.inner
    }
}
