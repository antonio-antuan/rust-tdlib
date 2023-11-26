use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Reports a false deletion of a message by aggressive anti-spam checks; requires administrator rights in the supergroup. Can be called only for messages from chatEventMessageDeleted with can_report_anti_spam_false_positive == true
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportSupergroupAntiSpamFalsePositive {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Supergroup identifier

    #[serde(default)]
    supergroup_id: i64,
    /// Identifier of the erroneously deleted message

    #[serde(default)]
    message_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ReportSupergroupAntiSpamFalsePositive {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ReportSupergroupAntiSpamFalsePositive {}

impl ReportSupergroupAntiSpamFalsePositive {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReportSupergroupAntiSpamFalsePositiveBuilder {
        let mut inner = ReportSupergroupAntiSpamFalsePositive::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "reportSupergroupAntiSpamFalsePositive".to_string();

        ReportSupergroupAntiSpamFalsePositiveBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i64 {
        self.supergroup_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }
}

#[doc(hidden)]
pub struct ReportSupergroupAntiSpamFalsePositiveBuilder {
    inner: ReportSupergroupAntiSpamFalsePositive,
}

#[deprecated]
pub type RTDReportSupergroupAntiSpamFalsePositiveBuilder =
    ReportSupergroupAntiSpamFalsePositiveBuilder;

impl ReportSupergroupAntiSpamFalsePositiveBuilder {
    pub fn build(&self) -> ReportSupergroupAntiSpamFalsePositive {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
        self.inner.supergroup_id = supergroup_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }
}

impl AsRef<ReportSupergroupAntiSpamFalsePositive> for ReportSupergroupAntiSpamFalsePositive {
    fn as_ref(&self) -> &ReportSupergroupAntiSpamFalsePositive {
        self
    }
}

impl AsRef<ReportSupergroupAntiSpamFalsePositive> for ReportSupergroupAntiSpamFalsePositiveBuilder {
    fn as_ref(&self) -> &ReportSupergroupAntiSpamFalsePositive {
        &self.inner
    }
}
