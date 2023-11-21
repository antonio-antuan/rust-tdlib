use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a Web App. Use getInternalLink with internalLinkTypeWebApp to share the Web App
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebApp {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Web App short name

    #[serde(default)]
    short_name: String,
    /// Web App title

    #[serde(default)]
    title: String,
    /// Describes a Web App. Use getInternalLink with internalLinkTypeWebApp to share the Web App

    #[serde(default)]
    description: String,
    /// Web App photo
    photo: Photo,
    /// Web App animation; may be null
    animation: Option<Animation>,
}

impl RObject for WebApp {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl WebApp {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> WebAppBuilder {
        let mut inner = WebApp::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        WebAppBuilder { inner }
    }

    pub fn short_name(&self) -> &String {
        &self.short_name
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn photo(&self) -> &Photo {
        &self.photo
    }

    pub fn animation(&self) -> &Option<Animation> {
        &self.animation
    }
}

#[doc(hidden)]
pub struct WebAppBuilder {
    inner: WebApp,
}

#[deprecated]
pub type RTDWebAppBuilder = WebAppBuilder;

impl WebAppBuilder {
    pub fn build(&self) -> WebApp {
        self.inner.clone()
    }

    pub fn short_name<T: AsRef<str>>(&mut self, short_name: T) -> &mut Self {
        self.inner.short_name = short_name.as_ref().to_string();
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
        self.inner.description = description.as_ref().to_string();
        self
    }

    pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
        self.inner.photo = photo.as_ref().clone();
        self
    }

    pub fn animation<T: AsRef<Animation>>(&mut self, animation: T) -> &mut Self {
        self.inner.animation = Some(animation.as_ref().clone());
        self
    }
}

impl AsRef<WebApp> for WebApp {
    fn as_ref(&self) -> &WebApp {
        self
    }
}

impl AsRef<WebApp> for WebAppBuilder {
    fn as_ref(&self) -> &WebApp {
        &self.inner
    }
}
