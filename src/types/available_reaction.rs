use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents an available reaction
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AvailableReaction {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text representation of the reaction
    reaction: String,
    /// True, if Telegram Premium is needed to send the reaction
    needs_premium: bool,
}

impl RObject for AvailableReaction {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl AvailableReaction {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDAvailableReactionBuilder {
        let mut inner = AvailableReaction::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDAvailableReactionBuilder { inner }
    }

    pub fn reaction(&self) -> &String {
        &self.reaction
    }

    pub fn needs_premium(&self) -> bool {
        self.needs_premium
    }
}

#[doc(hidden)]
pub struct RTDAvailableReactionBuilder {
    inner: AvailableReaction,
}

impl RTDAvailableReactionBuilder {
    pub fn build(&self) -> AvailableReaction {
        self.inner.clone()
    }

    pub fn reaction<T: AsRef<str>>(&mut self, reaction: T) -> &mut Self {
        self.inner.reaction = reaction.as_ref().to_string();
        self
    }

    pub fn needs_premium(&mut self, needs_premium: bool) -> &mut Self {
        self.inner.needs_premium = needs_premium;
        self
    }
}

impl AsRef<AvailableReaction> for AvailableReaction {
    fn as_ref(&self) -> &AvailableReaction {
        self
    }
}

impl AsRef<AvailableReaction> for RTDAvailableReactionBuilder {
    fn as_ref(&self) -> &AvailableReaction {
        &self.inner
    }
}
