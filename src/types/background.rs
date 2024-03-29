use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a chat background
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Background {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique background identifier

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    id: i64,
    /// True, if this is one of default backgrounds

    #[serde(default)]
    is_default: bool,
    /// True, if the background is dark and is recommended to be used with dark theme

    #[serde(default)]
    is_dark: bool,
    /// Unique background name

    #[serde(default)]
    name: String,
    /// Document with the background; may be null. Null only for filled backgrounds
    document: Option<Document>,
    /// Type of the background

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "BackgroundType::_is_default")]
    type_: BackgroundType,
}

impl RObject for Background {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Background {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> BackgroundBuilder {
        let mut inner = Background::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        BackgroundBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn is_default(&self) -> bool {
        self.is_default
    }

    pub fn is_dark(&self) -> bool {
        self.is_dark
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn document(&self) -> &Option<Document> {
        &self.document
    }

    pub fn type_(&self) -> &BackgroundType {
        &self.type_
    }
}

#[doc(hidden)]
pub struct BackgroundBuilder {
    inner: Background,
}

#[deprecated]
pub type RTDBackgroundBuilder = BackgroundBuilder;

impl BackgroundBuilder {
    pub fn build(&self) -> Background {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn is_default(&mut self, is_default: bool) -> &mut Self {
        self.inner.is_default = is_default;
        self
    }

    pub fn is_dark(&mut self, is_dark: bool) -> &mut Self {
        self.inner.is_dark = is_dark;
        self
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }

    pub fn document<T: AsRef<Document>>(&mut self, document: T) -> &mut Self {
        self.inner.document = Some(document.as_ref().clone());
        self
    }

    pub fn type_<T: AsRef<BackgroundType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }
}

impl AsRef<Background> for Background {
    fn as_ref(&self) -> &Background {
        self
    }
}

impl AsRef<Background> for BackgroundBuilder {
    fn as_ref(&self) -> &Background {
        &self.inner
    }
}
