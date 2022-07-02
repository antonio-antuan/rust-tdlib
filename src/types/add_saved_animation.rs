use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Manually adds a new animation to the list of saved animations. The new animation is added to the beginning of the list. If the animation was already in the list, it is removed first. Only non-secret video animations with MIME type "video/mp4" can be added to the list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddSavedAnimation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The animation file to be added. Only animations known to the server (i.e., successfully sent via a message) can be added to the list

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    animation: InputFile,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AddSavedAnimation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AddSavedAnimation {}

impl AddSavedAnimation {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AddSavedAnimationBuilder {
        let mut inner = AddSavedAnimation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "addSavedAnimation".to_string();

        AddSavedAnimationBuilder { inner }
    }

    pub fn animation(&self) -> &InputFile {
        &self.animation
    }
}

#[doc(hidden)]
pub struct AddSavedAnimationBuilder {
    inner: AddSavedAnimation,
}

#[deprecated]
pub type RTDAddSavedAnimationBuilder = AddSavedAnimationBuilder;

impl AddSavedAnimationBuilder {
    pub fn build(&self) -> AddSavedAnimation {
        self.inner.clone()
    }

    pub fn animation<T: AsRef<InputFile>>(&mut self, animation: T) -> &mut Self {
        self.inner.animation = animation.as_ref().clone();
        self
    }
}

impl AsRef<AddSavedAnimation> for AddSavedAnimation {
    fn as_ref(&self) -> &AddSavedAnimation {
        self
    }
}

impl AsRef<AddSavedAnimation> for AddSavedAnimationBuilder {
    fn as_ref(&self) -> &AddSavedAnimation {
        &self.inner
    }
}
