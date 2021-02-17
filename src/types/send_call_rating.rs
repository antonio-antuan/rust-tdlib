use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Sends a call rating
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendCallRating {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Call identifier
    call_id: i32,
    /// Call rating; 1-5
    rating: i32,
    /// An optional user comment if the rating is less than 5
    comment: String,
    /// List of the exact types of problems with the call, specified by the user
    problems: Vec<CallProblem>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SendCallRating {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SendCallRating {}

impl SendCallRating {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSendCallRatingBuilder {
        let mut inner = SendCallRating::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "sendCallRating".to_string();

        RTDSendCallRatingBuilder { inner }
    }

    pub fn call_id(&self) -> i32 {
        self.call_id
    }

    pub fn rating(&self) -> i32 {
        self.rating
    }

    pub fn comment(&self) -> &String {
        &self.comment
    }

    pub fn problems(&self) -> &Vec<CallProblem> {
        &self.problems
    }
}

#[doc(hidden)]
pub struct RTDSendCallRatingBuilder {
    inner: SendCallRating,
}

impl RTDSendCallRatingBuilder {
    pub fn build(&self) -> SendCallRating {
        self.inner.clone()
    }

    pub fn call_id(&mut self, call_id: i32) -> &mut Self {
        self.inner.call_id = call_id;
        self
    }

    pub fn rating(&mut self, rating: i32) -> &mut Self {
        self.inner.rating = rating;
        self
    }

    pub fn comment<T: AsRef<str>>(&mut self, comment: T) -> &mut Self {
        self.inner.comment = comment.as_ref().to_string();
        self
    }

    pub fn problems(&mut self, problems: Vec<CallProblem>) -> &mut Self {
        self.inner.problems = problems;
        self
    }
}

impl AsRef<SendCallRating> for SendCallRating {
    fn as_ref(&self) -> &SendCallRating {
        self
    }
}

impl AsRef<SendCallRating> for RTDSendCallRatingBuilder {
    fn as_ref(&self) -> &SendCallRating {
        &self.inner
    }
}
