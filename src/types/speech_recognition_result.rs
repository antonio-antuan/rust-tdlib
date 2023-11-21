use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes result of speech recognition in a voice note
pub trait TDSpeechRecognitionResult: Debug + RObject {}

/// Describes result of speech recognition in a voice note
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum SpeechRecognitionResult {
    #[doc(hidden)]
    #[default]
    _Default,
    /// The speech recognition failed
    #[serde(rename = "speechRecognitionResultError")]
    Error(SpeechRecognitionResultError),
    /// The speech recognition is ongoing
    #[serde(rename = "speechRecognitionResultPending")]
    Pending(SpeechRecognitionResultPending),
    /// The speech recognition successfully finished
    #[serde(rename = "speechRecognitionResultText")]
    Text(SpeechRecognitionResultText),
}

impl RObject for SpeechRecognitionResult {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            SpeechRecognitionResult::Error(t) => t.extra(),
            SpeechRecognitionResult::Pending(t) => t.extra(),
            SpeechRecognitionResult::Text(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            SpeechRecognitionResult::Error(t) => t.client_id(),
            SpeechRecognitionResult::Pending(t) => t.client_id(),
            SpeechRecognitionResult::Text(t) => t.client_id(),

            _ => None,
        }
    }
}

impl SpeechRecognitionResult {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, SpeechRecognitionResult::_Default)
    }
}

impl AsRef<SpeechRecognitionResult> for SpeechRecognitionResult {
    fn as_ref(&self) -> &SpeechRecognitionResult {
        self
    }
}

/// The speech recognition failed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpeechRecognitionResultError {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Recognition error
    error: Error,
}

impl RObject for SpeechRecognitionResultError {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSpeechRecognitionResult for SpeechRecognitionResultError {}

impl SpeechRecognitionResultError {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SpeechRecognitionResultErrorBuilder {
        let mut inner = SpeechRecognitionResultError::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SpeechRecognitionResultErrorBuilder { inner }
    }

    pub fn error(&self) -> &Error {
        &self.error
    }
}

#[doc(hidden)]
pub struct SpeechRecognitionResultErrorBuilder {
    inner: SpeechRecognitionResultError,
}

#[deprecated]
pub type RTDSpeechRecognitionResultErrorBuilder = SpeechRecognitionResultErrorBuilder;

impl SpeechRecognitionResultErrorBuilder {
    pub fn build(&self) -> SpeechRecognitionResultError {
        self.inner.clone()
    }

    pub fn error<T: AsRef<Error>>(&mut self, error: T) -> &mut Self {
        self.inner.error = error.as_ref().clone();
        self
    }
}

impl AsRef<SpeechRecognitionResultError> for SpeechRecognitionResultError {
    fn as_ref(&self) -> &SpeechRecognitionResultError {
        self
    }
}

impl AsRef<SpeechRecognitionResultError> for SpeechRecognitionResultErrorBuilder {
    fn as_ref(&self) -> &SpeechRecognitionResultError {
        &self.inner
    }
}

/// The speech recognition is ongoing
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpeechRecognitionResultPending {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Partially recognized text

    #[serde(default)]
    partial_text: String,
}

impl RObject for SpeechRecognitionResultPending {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSpeechRecognitionResult for SpeechRecognitionResultPending {}

impl SpeechRecognitionResultPending {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SpeechRecognitionResultPendingBuilder {
        let mut inner = SpeechRecognitionResultPending::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SpeechRecognitionResultPendingBuilder { inner }
    }

    pub fn partial_text(&self) -> &String {
        &self.partial_text
    }
}

#[doc(hidden)]
pub struct SpeechRecognitionResultPendingBuilder {
    inner: SpeechRecognitionResultPending,
}

#[deprecated]
pub type RTDSpeechRecognitionResultPendingBuilder = SpeechRecognitionResultPendingBuilder;

impl SpeechRecognitionResultPendingBuilder {
    pub fn build(&self) -> SpeechRecognitionResultPending {
        self.inner.clone()
    }

    pub fn partial_text<T: AsRef<str>>(&mut self, partial_text: T) -> &mut Self {
        self.inner.partial_text = partial_text.as_ref().to_string();
        self
    }
}

impl AsRef<SpeechRecognitionResultPending> for SpeechRecognitionResultPending {
    fn as_ref(&self) -> &SpeechRecognitionResultPending {
        self
    }
}

impl AsRef<SpeechRecognitionResultPending> for SpeechRecognitionResultPendingBuilder {
    fn as_ref(&self) -> &SpeechRecognitionResultPending {
        &self.inner
    }
}

/// The speech recognition successfully finished
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpeechRecognitionResultText {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Recognized text

    #[serde(default)]
    text: String,
}

impl RObject for SpeechRecognitionResultText {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSpeechRecognitionResult for SpeechRecognitionResultText {}

impl SpeechRecognitionResultText {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SpeechRecognitionResultTextBuilder {
        let mut inner = SpeechRecognitionResultText::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SpeechRecognitionResultTextBuilder { inner }
    }

    pub fn text(&self) -> &String {
        &self.text
    }
}

#[doc(hidden)]
pub struct SpeechRecognitionResultTextBuilder {
    inner: SpeechRecognitionResultText,
}

#[deprecated]
pub type RTDSpeechRecognitionResultTextBuilder = SpeechRecognitionResultTextBuilder;

impl SpeechRecognitionResultTextBuilder {
    pub fn build(&self) -> SpeechRecognitionResultText {
        self.inner.clone()
    }

    pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().to_string();
        self
    }
}

impl AsRef<SpeechRecognitionResultText> for SpeechRecognitionResultText {
    fn as_ref(&self) -> &SpeechRecognitionResultText {
        self
    }
}

impl AsRef<SpeechRecognitionResultText> for SpeechRecognitionResultTextBuilder {
    fn as_ref(&self) -> &SpeechRecognitionResultText {
        &self.inner
    }
}
