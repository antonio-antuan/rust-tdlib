use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns the list of commands supported by the bot for the given user scope and language; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetCommands {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The scope to which the commands are relevant; pass null to get commands in the default bot command scope

    #[serde(skip_serializing_if = "BotCommandScope::_is_default")]
    scope: BotCommandScope,
    /// A two-letter ISO 639-1 language code or an empty string
    language_code: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetCommands {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetCommands {}

impl GetCommands {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetCommandsBuilder {
        let mut inner = GetCommands::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getCommands".to_string();

        RTDGetCommandsBuilder { inner }
    }

    pub fn scope(&self) -> &BotCommandScope {
        &self.scope
    }

    pub fn language_code(&self) -> &String {
        &self.language_code
    }
}

#[doc(hidden)]
pub struct RTDGetCommandsBuilder {
    inner: GetCommands,
}

impl RTDGetCommandsBuilder {
    pub fn build(&self) -> GetCommands {
        self.inner.clone()
    }

    pub fn scope<T: AsRef<BotCommandScope>>(&mut self, scope: T) -> &mut Self {
        self.inner.scope = scope.as_ref().clone();
        self
    }

    pub fn language_code<T: AsRef<str>>(&mut self, language_code: T) -> &mut Self {
        self.inner.language_code = language_code.as_ref().to_string();
        self
    }
}

impl AsRef<GetCommands> for GetCommands {
    fn as_ref(&self) -> &GetCommands {
        self
    }
}

impl AsRef<GetCommands> for RTDGetCommandsBuilder {
    fn as_ref(&self) -> &GetCommands {
        &self.inner
    }
}