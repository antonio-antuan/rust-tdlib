use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Sets the list of commands supported by the bot; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetCommands {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of the bot's commands
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetCommandsBuilder {
        let mut inner = SetCommands::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setCommands".to_string();

        RTDSetCommandsBuilder { inner }
    }

    pub fn commands(&self) -> &Vec<BotCommand> {
        &self.commands
    }
}

#[doc(hidden)]
pub struct RTDSetCommandsBuilder {
    inner: SetCommands,
}

impl RTDSetCommandsBuilder {
    pub fn build(&self) -> SetCommands {
        self.inner.clone()
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

impl AsRef<SetCommands> for RTDSetCommandsBuilder {
    fn as_ref(&self) -> &SetCommands {
        &self.inner
    }
}
