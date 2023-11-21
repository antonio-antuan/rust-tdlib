use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes type of default reaction for the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetDefaultReactionType {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// New type of the default reaction

    #[serde(skip_serializing_if = "ReactionType::_is_default")]
    reaction_type: ReactionType,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetDefaultReactionType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetDefaultReactionType {}

impl SetDefaultReactionType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetDefaultReactionTypeBuilder {
        let mut inner = SetDefaultReactionType::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setDefaultReactionType".to_string();

        SetDefaultReactionTypeBuilder { inner }
    }

    pub fn reaction_type(&self) -> &ReactionType {
        &self.reaction_type
    }
}

#[doc(hidden)]
pub struct SetDefaultReactionTypeBuilder {
    inner: SetDefaultReactionType,
}

#[deprecated]
pub type RTDSetDefaultReactionTypeBuilder = SetDefaultReactionTypeBuilder;

impl SetDefaultReactionTypeBuilder {
    pub fn build(&self) -> SetDefaultReactionType {
        self.inner.clone()
    }

    pub fn reaction_type<T: AsRef<ReactionType>>(&mut self, reaction_type: T) -> &mut Self {
        self.inner.reaction_type = reaction_type.as_ref().clone();
        self
    }
}

impl AsRef<SetDefaultReactionType> for SetDefaultReactionType {
    fn as_ref(&self) -> &SetDefaultReactionType {
        self
    }
}

impl AsRef<SetDefaultReactionType> for SetDefaultReactionTypeBuilder {
    fn as_ref(&self) -> &SetDefaultReactionType {
        &self.inner
    }
}
