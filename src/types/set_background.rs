use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the background selected by the user; adds background to the list of installed backgrounds
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetBackground {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The input background to use; pass null to create a new filled backgrounds or to remove the current background

    #[serde(skip_serializing_if = "InputBackground::_is_default")]
    background: InputBackground,
    /// Background type; pass null to use the default type of the remote background or to remove the current background

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "BackgroundType::_is_default")]
    type_: BackgroundType,
    /// True, if the background is chosen for dark theme

    #[serde(default)]
    for_dark_theme: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetBackground {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetBackground {}

impl SetBackground {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetBackgroundBuilder {
        let mut inner = SetBackground::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setBackground".to_string();

        SetBackgroundBuilder { inner }
    }

    pub fn background(&self) -> &InputBackground {
        &self.background
    }

    pub fn type_(&self) -> &BackgroundType {
        &self.type_
    }

    pub fn for_dark_theme(&self) -> bool {
        self.for_dark_theme
    }
}

#[doc(hidden)]
pub struct SetBackgroundBuilder {
    inner: SetBackground,
}

#[deprecated]
pub type RTDSetBackgroundBuilder = SetBackgroundBuilder;

impl SetBackgroundBuilder {
    pub fn build(&self) -> SetBackground {
        self.inner.clone()
    }

    pub fn background<T: AsRef<InputBackground>>(&mut self, background: T) -> &mut Self {
        self.inner.background = background.as_ref().clone();
        self
    }

    pub fn type_<T: AsRef<BackgroundType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }

    pub fn for_dark_theme(&mut self, for_dark_theme: bool) -> &mut Self {
        self.inner.for_dark_theme = for_dark_theme;
        self
    }
}

impl AsRef<SetBackground> for SetBackground {
    fn as_ref(&self) -> &SetBackground {
        self
    }
}

impl AsRef<SetBackground> for SetBackgroundBuilder {
    fn as_ref(&self) -> &SetBackground {
        &self.inner
    }
}
