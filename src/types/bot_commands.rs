use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of bot commands
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotCommands {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Bot's user identifier

    #[serde(default)]
    bot_user_id: i64,
    /// List of bot commands

    #[serde(default)]
    commands: Vec<BotCommand>,
}

impl RObject for BotCommands {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl BotCommands {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> BotCommandsBuilder {
        let mut inner = BotCommands::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        BotCommandsBuilder { inner }
    }

    pub fn bot_user_id(&self) -> i64 {
        self.bot_user_id
    }

    pub fn commands(&self) -> &Vec<BotCommand> {
        &self.commands
    }
}

#[doc(hidden)]
pub struct BotCommandsBuilder {
    inner: BotCommands,
}

#[deprecated]
pub type RTDBotCommandsBuilder = BotCommandsBuilder;

impl BotCommandsBuilder {
    pub fn build(&self) -> BotCommands {
        self.inner.clone()
    }

    pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
        self.inner.bot_user_id = bot_user_id;
        self
    }

    pub fn commands(&mut self, commands: Vec<BotCommand>) -> &mut Self {
        self.inner.commands = commands;
        self
    }
}

impl AsRef<BotCommands> for BotCommands {
    fn as_ref(&self) -> &BotCommands {
        self
    }
}

impl AsRef<BotCommands> for BotCommandsBuilder {
    fn as_ref(&self) -> &BotCommands {
        &self.inner
    }
}
