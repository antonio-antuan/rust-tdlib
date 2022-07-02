use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of animations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Animations {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of animations

    #[serde(default)]
    animations: Vec<Animation>,
}

impl RObject for Animations {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Animations {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AnimationsBuilder {
        let mut inner = Animations::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AnimationsBuilder { inner }
    }

    pub fn animations(&self) -> &Vec<Animation> {
        &self.animations
    }
}

#[doc(hidden)]
pub struct AnimationsBuilder {
    inner: Animations,
}

#[deprecated]
pub type RTDAnimationsBuilder = AnimationsBuilder;

impl AnimationsBuilder {
    pub fn build(&self) -> Animations {
        self.inner.clone()
    }

    pub fn animations(&mut self, animations: Vec<Animation>) -> &mut Self {
        self.inner.animations = animations;
        self
    }
}

impl AsRef<Animations> for Animations {
    fn as_ref(&self) -> &Animations {
        self
    }
}

impl AsRef<Animations> for AnimationsBuilder {
    fn as_ref(&self) -> &Animations {
        &self.inner
    }
}
