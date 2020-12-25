
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Represents commands supported by a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotCommand {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text of the bot command
  command: String,
  /// Represents commands supported by a bot
  description: String,
  
}

impl RObject for BotCommand {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "botCommand" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl BotCommand {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDBotCommandBuilder {
    let mut inner = BotCommand::default();
    inner.td_name = "botCommand".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDBotCommandBuilder { inner }
  }

  pub fn command(&self) -> &String { &self.command }

  pub fn description(&self) -> &String { &self.description }

}

#[doc(hidden)]
pub struct RTDBotCommandBuilder {
  inner: BotCommand
}

impl RTDBotCommandBuilder {
  pub fn build(&self) -> BotCommand { self.inner.clone() }

   
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
  fn as_ref(&self) -> &BotCommand { self }
}

impl AsRef<BotCommand> for RTDBotCommandBuilder {
  fn as_ref(&self) -> &BotCommand { &self.inner }
}



