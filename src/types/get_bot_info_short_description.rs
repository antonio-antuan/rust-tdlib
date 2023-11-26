use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns the text shown on a bot's profile page and sent together with the link when users share the bot in the given language. Can be called only if userTypeBot.can_be_edited == true
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBotInfoShortDescription {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the target bot

    #[serde(default)]
    bot_user_id: i64,
    /// A two-letter ISO 639-1 language code or an empty string

    #[serde(default)]
    language_code: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetBotInfoShortDescription {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetBotInfoShortDescription {}

impl GetBotInfoShortDescription {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetBotInfoShortDescriptionBuilder {
        let mut inner = GetBotInfoShortDescription::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getBotInfoShortDescription".to_string();

        GetBotInfoShortDescriptionBuilder { inner }
    }

    pub fn bot_user_id(&self) -> i64 {
        self.bot_user_id
    }

    pub fn language_code(&self) -> &String {
        &self.language_code
    }
}

#[doc(hidden)]
pub struct GetBotInfoShortDescriptionBuilder {
    inner: GetBotInfoShortDescription,
}

#[deprecated]
pub type RTDGetBotInfoShortDescriptionBuilder = GetBotInfoShortDescriptionBuilder;

impl GetBotInfoShortDescriptionBuilder {
    pub fn build(&self) -> GetBotInfoShortDescription {
        self.inner.clone()
    }

    pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
        self.inner.bot_user_id = bot_user_id;
        self
    }

    pub fn language_code<T: AsRef<str>>(&mut self, language_code: T) -> &mut Self {
        self.inner.language_code = language_code.as_ref().to_string();
        self
    }
}

impl AsRef<GetBotInfoShortDescription> for GetBotInfoShortDescription {
    fn as_ref(&self) -> &GetBotInfoShortDescription {
        self
    }
}

impl AsRef<GetBotInfoShortDescription> for GetBotInfoShortDescriptionBuilder {
    fn as_ref(&self) -> &GetBotInfoShortDescription {
        &self.inner
    }
}
