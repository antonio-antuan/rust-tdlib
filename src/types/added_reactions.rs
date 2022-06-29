use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of reactions added to a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddedReactions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The total number of found reactions

    #[serde(default)]
    total_count: i32,
    /// The list of added reactions

    #[serde(default)]
    reactions: Vec<AddedReaction>,
    /// The offset for the next request. If empty, there are no more results

    #[serde(default)]
    next_offset: String,
}

impl RObject for AddedReactions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl AddedReactions {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDAddedReactionsBuilder {
        let mut inner = AddedReactions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDAddedReactionsBuilder { inner }
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn reactions(&self) -> &Vec<AddedReaction> {
        &self.reactions
    }

    pub fn next_offset(&self) -> &String {
        &self.next_offset
    }
}

#[doc(hidden)]
pub struct RTDAddedReactionsBuilder {
    inner: AddedReactions,
}

impl RTDAddedReactionsBuilder {
    pub fn build(&self) -> AddedReactions {
        self.inner.clone()
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }

    pub fn reactions(&mut self, reactions: Vec<AddedReaction>) -> &mut Self {
        self.inner.reactions = reactions;
        self
    }

    pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
        self.inner.next_offset = next_offset.as_ref().to_string();
        self
    }
}

impl AsRef<AddedReactions> for AddedReactions {
    fn as_ref(&self) -> &AddedReactions {
        self
    }
}

impl AsRef<AddedReactions> for RTDAddedReactionsBuilder {
    fn as_ref(&self) -> &AddedReactions {
        &self.inner
    }
}
