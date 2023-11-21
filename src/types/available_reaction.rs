use crate::errors::Result;
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
    /// Type of the reaction

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "ReactionType::_is_default")]
    type_: ReactionType,
    /// True, if Telegram Premium is needed to send the reaction

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AvailableReactionBuilder {
        let mut inner = AvailableReaction::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AvailableReactionBuilder { inner }
    }

    pub fn type_(&self) -> &ReactionType {
        &self.type_
    }

    pub fn needs_premium(&self) -> bool {
        self.needs_premium
    }
}

#[doc(hidden)]
pub struct AvailableReactionBuilder {
    inner: AvailableReaction,
}

#[deprecated]
pub type RTDAvailableReactionBuilder = AvailableReactionBuilder;

impl AvailableReactionBuilder {
    pub fn build(&self) -> AvailableReaction {
        self.inner.clone()
    }

    pub fn type_<T: AsRef<ReactionType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
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

impl AsRef<AvailableReaction> for AvailableReactionBuilder {
    fn as_ref(&self) -> &AvailableReaction {
        &self.inner
    }
}
