use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a command supported by a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotCommand {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text of the bot command

    #[serde(default)]
    command: String,
    /// Represents a command supported by a bot

    #[serde(default)]
    description: String,
}

impl RObject for BotCommand {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl BotCommand {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> BotCommandBuilder {
        let mut inner = BotCommand::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        BotCommandBuilder { inner }
    }

    pub fn command(&self) -> &String {
        &self.command
    }

    pub fn description(&self) -> &String {
        &self.description
    }
}

#[doc(hidden)]
pub struct BotCommandBuilder {
    inner: BotCommand,
}

#[deprecated]
pub type RTDBotCommandBuilder = BotCommandBuilder;

impl BotCommandBuilder {
    pub fn build(&self) -> BotCommand {
        self.inner.clone()
    }

    pub fn command<T: AsRef<str>>(&mut self, command: T) -> &mut Self {
        self.inner.command = command.as_ref().to_string();
        self
    }

    pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
        self.inner.description = description.as_ref().to_string();
        self
    }
}

impl AsRef<BotCommand> for BotCommand {
    fn as_ref(&self) -> &BotCommand {
        self
    }
}

impl AsRef<BotCommand> for BotCommandBuilder {
    fn as_ref(&self) -> &BotCommand {
        &self.inner
    }
}
