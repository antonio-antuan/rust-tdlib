use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Reports messages in a supergroup as spam; requires administrator rights in the supergroup
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportSupergroupSpam {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Supergroup identifier

    #[serde(default)]
    supergroup_id: i64,
    /// Identifiers of messages to report

    #[serde(default)]
    message_ids: Vec<i64>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ReportSupergroupSpam {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ReportSupergroupSpam {}

impl ReportSupergroupSpam {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReportSupergroupSpamBuilder {
        let mut inner = ReportSupergroupSpam::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "reportSupergroupSpam".to_string();

        ReportSupergroupSpamBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i64 {
        self.supergroup_id
    }

    pub fn message_ids(&self) -> &Vec<i64> {
        &self.message_ids
    }
}

#[doc(hidden)]
pub struct ReportSupergroupSpamBuilder {
    inner: ReportSupergroupSpam,
}

#[deprecated]
pub type RTDReportSupergroupSpamBuilder = ReportSupergroupSpamBuilder;

impl ReportSupergroupSpamBuilder {
    pub fn build(&self) -> ReportSupergroupSpam {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
        self.inner.supergroup_id = supergroup_id;
        self
    }

    pub fn message_ids(&mut self, message_ids: Vec<i64>) -> &mut Self {
        self.inner.message_ids = message_ids;
        self
    }
}

impl AsRef<ReportSupergroupSpam> for ReportSupergroupSpam {
    fn as_ref(&self) -> &ReportSupergroupSpam {
        self
    }
}

impl AsRef<ReportSupergroupSpam> for ReportSupergroupSpamBuilder {
    fn as_ref(&self) -> &ReportSupergroupSpam {
        &self.inner
    }
}
