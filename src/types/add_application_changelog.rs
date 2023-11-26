use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Adds server-provided application changelog as messages to the chat 777000 (Telegram) or as a stories; for official applications only. Returns a 404 error if nothing changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddApplicationChangelog {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The previous application version

    #[serde(default)]
    previous_application_version: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AddApplicationChangelog {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AddApplicationChangelog {}

impl AddApplicationChangelog {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AddApplicationChangelogBuilder {
        let mut inner = AddApplicationChangelog::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "addApplicationChangelog".to_string();

        AddApplicationChangelogBuilder { inner }
    }

    pub fn previous_application_version(&self) -> &String {
        &self.previous_application_version
    }
}

#[doc(hidden)]
pub struct AddApplicationChangelogBuilder {
    inner: AddApplicationChangelog,
}

#[deprecated]
pub type RTDAddApplicationChangelogBuilder = AddApplicationChangelogBuilder;

impl AddApplicationChangelogBuilder {
    pub fn build(&self) -> AddApplicationChangelog {
        self.inner.clone()
    }

    pub fn previous_application_version<T: AsRef<str>>(
        &mut self,
        previous_application_version: T,
    ) -> &mut Self {
        self.inner.previous_application_version = previous_application_version.as_ref().to_string();
        self
    }
}

impl AsRef<AddApplicationChangelog> for AddApplicationChangelog {
    fn as_ref(&self) -> &AddApplicationChangelog {
        self
    }
}

impl AsRef<AddApplicationChangelog> for AddApplicationChangelogBuilder {
    fn as_ref(&self) -> &AddApplicationChangelog {
        &self.inner
    }
}
