use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of backgrounds
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Backgrounds {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A list of backgrounds
    backgrounds: Vec<Background>,
}

impl RObject for Backgrounds {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Backgrounds {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDBackgroundsBuilder {
        let mut inner = Backgrounds::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDBackgroundsBuilder { inner }
    }

    pub fn backgrounds(&self) -> &Vec<Background> {
        &self.backgrounds
    }
}

#[doc(hidden)]
pub struct RTDBackgroundsBuilder {
    inner: Backgrounds,
}

impl RTDBackgroundsBuilder {
    pub fn build(&self) -> Backgrounds {
        self.inner.clone()
    }

    pub fn backgrounds(&mut self, backgrounds: Vec<Background>) -> &mut Self {
        self.inner.backgrounds = backgrounds;
        self
    }
}

impl AsRef<Backgrounds> for Backgrounds {
    fn as_ref(&self) -> &Backgrounds {
        self
    }
}

impl AsRef<Backgrounds> for RTDBackgroundsBuilder {
    fn as_ref(&self) -> &Backgrounds {
        &self.inner
    }
}
