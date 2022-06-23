use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of available reactions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AvailableReactions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of reactions
    reactions: Vec<AvailableReaction>,
}

impl RObject for AvailableReactions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl AvailableReactions {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDAvailableReactionsBuilder {
        let mut inner = AvailableReactions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDAvailableReactionsBuilder { inner }
    }

    pub fn reactions(&self) -> &Vec<AvailableReaction> {
        &self.reactions
    }
}

#[doc(hidden)]
pub struct RTDAvailableReactionsBuilder {
    inner: AvailableReactions,
}

impl RTDAvailableReactionsBuilder {
    pub fn build(&self) -> AvailableReactions {
        self.inner.clone()
    }

    pub fn reactions(&mut self, reactions: Vec<AvailableReaction>) -> &mut Self {
        self.inner.reactions = reactions;
        self
    }
}

impl AsRef<AvailableReactions> for AvailableReactions {
    fn as_ref(&self) -> &AvailableReactions {
        self
    }
}

impl AsRef<AvailableReactions> for RTDAvailableReactionsBuilder {
    fn as_ref(&self) -> &AvailableReactions {
        &self.inner
    }
}
