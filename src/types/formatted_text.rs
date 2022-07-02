use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// A text with some entities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FormattedText {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The text

    #[serde(default)]
    text: String,
    /// Entities contained in the text. Entities can be nested, but must not mutually intersect with each other. Pre, Code and PreCode entities can't contain other entities. Bold, Italic, Underline and Strikethrough entities can contain and to be contained in all other entities. All other entities can't contain each other

    #[serde(default)]
    entities: Vec<TextEntity>,
}

impl RObject for FormattedText {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl FormattedText {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FormattedTextBuilder {
        let mut inner = FormattedText::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FormattedTextBuilder { inner }
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn entities(&self) -> &Vec<TextEntity> {
        &self.entities
    }
}

#[doc(hidden)]
pub struct FormattedTextBuilder {
    inner: FormattedText,
}

#[deprecated]
pub type RTDFormattedTextBuilder = FormattedTextBuilder;

impl FormattedTextBuilder {
    pub fn build(&self) -> FormattedText {
        self.inner.clone()
    }

    pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().to_string();
        self
    }

    pub fn entities(&mut self, entities: Vec<TextEntity>) -> &mut Self {
        self.inner.entities = entities;
        self
    }
}

impl AsRef<FormattedText> for FormattedText {
    fn as_ref(&self) -> &FormattedText {
        self
    }
}

impl AsRef<FormattedText> for FormattedTextBuilder {
    fn as_ref(&self) -> &FormattedText {
        &self.inner
    }
}
