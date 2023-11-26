use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of story areas to be added
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputStoryAreas {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of 0-10 input story areas

    #[serde(default)]
    areas: Vec<InputStoryArea>,
}

impl RObject for InputStoryAreas {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl InputStoryAreas {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputStoryAreasBuilder {
        let mut inner = InputStoryAreas::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputStoryAreasBuilder { inner }
    }

    pub fn areas(&self) -> &Vec<InputStoryArea> {
        &self.areas
    }
}

#[doc(hidden)]
pub struct InputStoryAreasBuilder {
    inner: InputStoryAreas,
}

#[deprecated]
pub type RTDInputStoryAreasBuilder = InputStoryAreasBuilder;

impl InputStoryAreasBuilder {
    pub fn build(&self) -> InputStoryAreas {
        self.inner.clone()
    }

    pub fn areas(&mut self, areas: Vec<InputStoryArea>) -> &mut Self {
        self.inner.areas = areas;
        self
    }
}

impl AsRef<InputStoryAreas> for InputStoryAreas {
    fn as_ref(&self) -> &InputStoryAreas {
        self
    }
}

impl AsRef<InputStoryAreas> for InputStoryAreasBuilder {
    fn as_ref(&self) -> &InputStoryAreas {
        &self.inner
    }
}
