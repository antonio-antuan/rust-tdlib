use crate::errors::*;
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDAnimationsBuilder {
        let mut inner = Animations::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDAnimationsBuilder { inner }
    }

    pub fn animations(&self) -> &Vec<Animation> {
        &self.animations
    }
}

#[doc(hidden)]
pub struct RTDAnimationsBuilder {
    inner: Animations,
}

impl RTDAnimationsBuilder {
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

impl AsRef<Animations> for RTDAnimationsBuilder {
    fn as_ref(&self) -> &Animations {
        &self.inner
    }
}
