use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes the type of a poll
pub trait TDPollType: Debug + RObject {}

/// Describes the type of a poll
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PollType {
    #[doc(hidden)]
    _Default(()),
    /// A poll in quiz mode, which has exactly one correct answer option and can be answered only once
    Quiz(PollTypeQuiz),
    /// A regular poll
    Regular(PollTypeRegular),
}

impl Default for PollType {
    fn default() -> Self {
        PollType::_Default(())
    }
}

impl<'de> Deserialize<'de> for PollType {
    fn deserialize<D>(deserializer: D) -> Result<PollType, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          PollType,
          (pollTypeQuiz, Quiz);
          (pollTypeRegular, Regular);

        )(deserializer)
    }
}

impl RObject for PollType {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            PollType::Quiz(t) => t.td_name(),
            PollType::Regular(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            PollType::Quiz(t) => t.extra(),
            PollType::Regular(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
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
        matches!(self, PollType::_Default(_))
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
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
    fn td_name(&self) -> &'static str {
        "pollTypeQuiz"
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

impl TDPollType for PollTypeQuiz {}

impl PollTypeQuiz {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPollTypeQuizBuilder {
        let mut inner = PollTypeQuiz::default();
        inner.td_name = "pollTypeQuiz".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
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
    fn td_name(&self) -> &'static str {
        "pollTypeRegular"
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

impl TDPollType for PollTypeRegular {}

impl PollTypeRegular {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPollTypeRegularBuilder {
        let mut inner = PollTypeRegular::default();
        inner.td_name = "pollTypeRegular".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
