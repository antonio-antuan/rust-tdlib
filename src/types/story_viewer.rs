use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a viewer of a story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryViewer {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifier of the viewer

    #[serde(default)]
    user_id: i64,
    /// Approximate point in time (Unix timestamp) when the story was viewed

    #[serde(default)]
    view_date: i32,
    /// Block list to which the user is added; may be null if none
    block_list: Option<BlockList>,
    /// Type of the reaction that was chosen by the user; may be null if none
    chosen_reaction_type: Option<ReactionType>,
}

impl RObject for StoryViewer {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl StoryViewer {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StoryViewerBuilder {
        let mut inner = StoryViewer::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StoryViewerBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn view_date(&self) -> i32 {
        self.view_date
    }

    pub fn block_list(&self) -> &Option<BlockList> {
        &self.block_list
    }

    pub fn chosen_reaction_type(&self) -> &Option<ReactionType> {
        &self.chosen_reaction_type
    }
}

#[doc(hidden)]
pub struct StoryViewerBuilder {
    inner: StoryViewer,
}

#[deprecated]
pub type RTDStoryViewerBuilder = StoryViewerBuilder;

impl StoryViewerBuilder {
    pub fn build(&self) -> StoryViewer {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn view_date(&mut self, view_date: i32) -> &mut Self {
        self.inner.view_date = view_date;
        self
    }

    pub fn block_list<T: AsRef<BlockList>>(&mut self, block_list: T) -> &mut Self {
        self.inner.block_list = Some(block_list.as_ref().clone());
        self
    }

    pub fn chosen_reaction_type<T: AsRef<ReactionType>>(
        &mut self,
        chosen_reaction_type: T,
    ) -> &mut Self {
        self.inner.chosen_reaction_type = Some(chosen_reaction_type.as_ref().clone());
        self
    }
}

impl AsRef<StoryViewer> for StoryViewer {
    fn as_ref(&self) -> &StoryViewer {
        self
    }
}

impl AsRef<StoryViewer> for StoryViewerBuilder {
    fn as_ref(&self) -> &StoryViewer {
        &self.inner
    }
}
