use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of stories
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Stories {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Approximate total number of stories found

    #[serde(default)]
    total_count: i32,
    /// The list of stories

    #[serde(default)]
    stories: Vec<Story>,
}

impl RObject for Stories {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Stories {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StoriesBuilder {
        let mut inner = Stories::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StoriesBuilder { inner }
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn stories(&self) -> &Vec<Story> {
        &self.stories
    }
}

#[doc(hidden)]
pub struct StoriesBuilder {
    inner: Stories,
}

#[deprecated]
pub type RTDStoriesBuilder = StoriesBuilder;

impl StoriesBuilder {
    pub fn build(&self) -> Stories {
        self.inner.clone()
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }

    pub fn stories(&mut self, stories: Vec<Story>) -> &mut Self {
        self.inner.stories = stories;
        self
    }
}

impl AsRef<Stories> for Stories {
    fn as_ref(&self) -> &Stories {
        self
    }
}

impl AsRef<Stories> for StoriesBuilder {
    fn as_ref(&self) -> &Stories {
        &self.inner
    }
}
