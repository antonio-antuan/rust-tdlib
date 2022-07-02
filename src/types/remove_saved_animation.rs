use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Removes an animation from the list of saved animations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RemoveSavedAnimation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Animation file to be removed

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    animation: InputFile,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for RemoveSavedAnimation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for RemoveSavedAnimation {}

impl RemoveSavedAnimation {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RemoveSavedAnimationBuilder {
        let mut inner = RemoveSavedAnimation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "removeSavedAnimation".to_string();

        RemoveSavedAnimationBuilder { inner }
    }

    pub fn animation(&self) -> &InputFile {
        &self.animation
    }
}

#[doc(hidden)]
pub struct RemoveSavedAnimationBuilder {
    inner: RemoveSavedAnimation,
}

#[deprecated]
pub type RTDRemoveSavedAnimationBuilder = RemoveSavedAnimationBuilder;

impl RemoveSavedAnimationBuilder {
    pub fn build(&self) -> RemoveSavedAnimation {
        self.inner.clone()
    }

    pub fn animation<T: AsRef<InputFile>>(&mut self, animation: T) -> &mut Self {
        self.inner.animation = animation.as_ref().clone();
        self
    }
}

impl AsRef<RemoveSavedAnimation> for RemoveSavedAnimation {
    fn as_ref(&self) -> &RemoveSavedAnimation {
        self
    }
}

impl AsRef<RemoveSavedAnimation> for RemoveSavedAnimationBuilder {
    fn as_ref(&self) -> &RemoveSavedAnimation {
        &self.inner
    }
}
