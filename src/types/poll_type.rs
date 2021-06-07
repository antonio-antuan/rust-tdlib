use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes the type of a poll
pub trait TDPollType: Debug + RObject {}

/// Describes the type of a poll
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PollType {
    #[doc(hidden)]
    _Default,
    /// A poll in quiz mode, which has exactly one correct answer option and can be answered only once
    #[serde(rename(serialize = "pollTypeQuiz", deserialize = "pollTypeQuiz"))]
    Quiz(PollTypeQuiz),
    /// A regular poll
    #[serde(rename(serialize = "pollTypeRegular", deserialize = "pollTypeRegular"))]
    Regular(PollTypeRegular),
}

impl Default for PollType {
    fn default() -> Self {
        PollType::_Default
    }
}

impl RObject for PollType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            PollType::Quiz(t) => t.extra(),
            PollType::Regular(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            PollType::Quiz(t) => t.client_id(),
            PollType::Regular(t) => t.client_id(),

            _ => None,
        }
    }
}

impl PollType {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, PollType::_Default)
    }
}

impl AsRef<PollType> for PollType {
    fn as_ref(&self) -> &PollType {
        self
    }
}

/// A poll in quiz mode, which has exactly one correct answer option and can be answered only once
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollTypeQuiz {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// 0-based identifier of the correct answer option; 1 for a yet unanswered poll
    correct_option_id: i32,
    /// Text that is shown when the user chooses an incorrect answer or taps on the lamp icon, 0-200 characters with at most 2 line feeds; empty for a yet unanswered poll
    explanation: FormattedText,
}

impl RObject for PollTypeQuiz {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPollType for PollTypeQuiz {}

impl PollTypeQuiz {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPollTypeQuizBuilder {
        let mut inner = PollTypeQuiz::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPollTypeQuizBuilder { inner }
    }

    pub fn correct_option_id(&self) -> i32 {
        self.correct_option_id
    }

    pub fn explanation(&self) -> &FormattedText {
        &self.explanation
    }
}

#[doc(hidden)]
pub struct RTDPollTypeQuizBuilder {
    inner: PollTypeQuiz,
}

impl RTDPollTypeQuizBuilder {
    pub fn build(&self) -> PollTypeQuiz {
        self.inner.clone()
    }

    pub fn correct_option_id(&mut self, correct_option_id: i32) -> &mut Self {
        self.inner.correct_option_id = correct_option_id;
        self
    }

    pub fn explanation<T: AsRef<FormattedText>>(&mut self, explanation: T) -> &mut Self {
        self.inner.explanation = explanation.as_ref().clone();
        self
    }
}

impl AsRef<PollTypeQuiz> for PollTypeQuiz {
    fn as_ref(&self) -> &PollTypeQuiz {
        self
    }
}

impl AsRef<PollTypeQuiz> for RTDPollTypeQuizBuilder {
    fn as_ref(&self) -> &PollTypeQuiz {
        &self.inner
    }
}

/// A regular poll
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PollTypeRegular {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if multiple answer options can be chosen simultaneously
    allow_multiple_answers: bool,
}

impl RObject for PollTypeRegular {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPollType for PollTypeRegular {}

impl PollTypeRegular {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPollTypeRegularBuilder {
        let mut inner = PollTypeRegular::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPollTypeRegularBuilder { inner }
    }

    pub fn allow_multiple_answers(&self) -> bool {
        self.allow_multiple_answers
    }
}

#[doc(hidden)]
pub struct RTDPollTypeRegularBuilder {
    inner: PollTypeRegular,
}

impl RTDPollTypeRegularBuilder {
    pub fn build(&self) -> PollTypeRegular {
        self.inner.clone()
    }

    pub fn allow_multiple_answers(&mut self, allow_multiple_answers: bool) -> &mut Self {
        self.inner.allow_multiple_answers = allow_multiple_answers;
        self
    }
}

impl AsRef<PollTypeRegular> for PollTypeRegular {
    fn as_ref(&self) -> &PollTypeRegular {
        self
    }
}

impl AsRef<PollTypeRegular> for RTDPollTypeRegularBuilder {
    fn as_ref(&self) -> &PollTypeRegular {
        &self.inner
    }
}
