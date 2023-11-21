use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes order of active usernames of a bot. Can be called only if userTypeBot.can_be_edited == true
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReorderBotActiveUsernames {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the target bot

    #[serde(default)]
    bot_user_id: i64,
    /// The new order of active usernames. All currently active usernames must be specified

    #[serde(default)]
    usernames: Vec<String>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ReorderBotActiveUsernames {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ReorderBotActiveUsernames {}

impl ReorderBotActiveUsernames {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReorderBotActiveUsernamesBuilder {
        let mut inner = ReorderBotActiveUsernames::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "reorderBotActiveUsernames".to_string();

        ReorderBotActiveUsernamesBuilder { inner }
    }

    pub fn bot_user_id(&self) -> i64 {
        self.bot_user_id
    }

    pub fn usernames(&self) -> &Vec<String> {
        &self.usernames
    }
}

#[doc(hidden)]
pub struct ReorderBotActiveUsernamesBuilder {
    inner: ReorderBotActiveUsernames,
}

#[deprecated]
pub type RTDReorderBotActiveUsernamesBuilder = ReorderBotActiveUsernamesBuilder;

impl ReorderBotActiveUsernamesBuilder {
    pub fn build(&self) -> ReorderBotActiveUsernames {
        self.inner.clone()
    }

    pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
        self.inner.bot_user_id = bot_user_id;
        self
    }

    pub fn usernames(&mut self, usernames: Vec<String>) -> &mut Self {
        self.inner.usernames = usernames;
        self
    }
}

impl AsRef<ReorderBotActiveUsernames> for ReorderBotActiveUsernames {
    fn as_ref(&self) -> &ReorderBotActiveUsernames {
        self
    }
}

impl AsRef<ReorderBotActiveUsernames> for ReorderBotActiveUsernamesBuilder {
    fn as_ref(&self) -> &ReorderBotActiveUsernames {
        &self.inner
    }
}
