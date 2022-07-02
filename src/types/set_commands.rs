use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sets the list of commands supported by the bot for the given user scope and language; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetCommands {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The scope to which the commands are relevant; pass null to change commands in the default bot command scope

    #[serde(skip_serializing_if = "BotCommandScope::_is_default")]
    scope: BotCommandScope,
    /// A two-letter ISO 639-1 country code. If empty, the commands will be applied to all users from the given scope, for which language there are no dedicated commands

    #[serde(default)]
    language_code: String,
    /// List of the bot's commands

    #[serde(default)]
    commands: Vec<BotCommand>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetCommands {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetCommands {}

impl SetCommands {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetCommandsBuilder {
        let mut inner = SetCommands::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setCommands".to_string();

        SetCommandsBuilder { inner }
    }

    pub fn scope(&self) -> &BotCommandScope {
        &self.scope
    }

    pub fn language_code(&self) -> &String {
        &self.language_code
    }

    pub fn commands(&self) -> &Vec<BotCommand> {
        &self.commands
    }
}

#[doc(hidden)]
pub struct SetCommandsBuilder {
    inner: SetCommands,
}

#[deprecated]
pub type RTDSetCommandsBuilder = SetCommandsBuilder;

impl SetCommandsBuilder {
    pub fn build(&self) -> SetCommands {
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

    pub fn commands(&mut self, commands: Vec<BotCommand>) -> &mut Self {
        self.inner.commands = commands;
        self
    }
}

impl AsRef<SetCommands> for SetCommands {
    fn as_ref(&self) -> &SetCommands {
        self
    }
}

impl AsRef<SetCommands> for SetCommandsBuilder {
    fn as_ref(&self) -> &SetCommands {
        &self.inner
    }
}
