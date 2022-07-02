use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains some text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Text {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text

    #[serde(default)]
    text: String,
}

impl RObject for Text {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Text {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TextBuilder {
        let mut inner = Text::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TextBuilder { inner }
    }

    pub fn text(&self) -> &String {
        &self.text
    }
}

#[doc(hidden)]
pub struct TextBuilder {
    inner: Text,
}

#[deprecated]
pub type RTDTextBuilder = TextBuilder;

impl TextBuilder {
    pub fn build(&self) -> Text {
        self.inner.clone()
    }

    pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().to_string();
        self
    }
}

impl AsRef<Text> for Text {
    fn as_ref(&self) -> &Text {
        self
    }
}

impl AsRef<Text> for TextBuilder {
    fn as_ref(&self) -> &Text {
        &self.inner
    }
}
