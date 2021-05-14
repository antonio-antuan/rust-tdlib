use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Reports some messages from a user in a supergroup as spam; requires administrator rights in the supergroup
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportSupergroupSpam {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Supergroup identifier
    supergroup_id: i32,
    /// User identifier
    user_id: i32,
    /// Identifiers of messages sent in the supergroup by the user. This list must be non-empty
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDReportSupergroupSpamBuilder {
        let mut inner = ReportSupergroupSpam::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "reportSupergroupSpam".to_string();

        RTDReportSupergroupSpamBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i32 {
        self.supergroup_id
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn message_ids(&self) -> &Vec<i64> {
        &self.message_ids
    }
}

#[doc(hidden)]
pub struct RTDReportSupergroupSpamBuilder {
    inner: ReportSupergroupSpam,
}

impl RTDReportSupergroupSpamBuilder {
    pub fn build(&self) -> ReportSupergroupSpam {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self {
        self.inner.supergroup_id = supergroup_id;
        self
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
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

impl AsRef<ReportSupergroupSpam> for RTDReportSupergroupSpamBuilder {
    fn as_ref(&self) -> &ReportSupergroupSpam {
        &self.inner
    }
}
