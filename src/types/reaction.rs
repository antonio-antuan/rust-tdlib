use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains stickers which must be used for reaction animation rendering
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Reaction {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text representation of the reaction

    #[serde(default)]
    reaction: String,
    /// Reaction title

    #[serde(default)]
    title: String,
    /// True, if the reaction can be added to new messages and enabled in chats

    #[serde(default)]
    is_active: bool,
    /// True, if the reaction is available only for Premium users

    #[serde(default)]
    is_premium: bool,
    /// Static icon for the reaction
    static_icon: Sticker,
    /// Appear animation for the reaction
    appear_animation: Sticker,
    /// Select animation for the reaction
    select_animation: Sticker,
    /// Activate animation for the reaction
    activate_animation: Sticker,
    /// Effect animation for the reaction
    effect_animation: Sticker,
    /// Around animation for the reaction; may be null
    around_animation: Option<Sticker>,
    /// Center animation for the reaction; may be null
    center_animation: Option<Sticker>,
}

impl RObject for Reaction {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Reaction {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDReactionBuilder {
        let mut inner = Reaction::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDReactionBuilder { inner }
    }

    pub fn reaction(&self) -> &String {
        &self.reaction
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn is_premium(&self) -> bool {
        self.is_premium
    }

    pub fn static_icon(&self) -> &Sticker {
        &self.static_icon
    }

    pub fn appear_animation(&self) -> &Sticker {
        &self.appear_animation
    }

    pub fn select_animation(&self) -> &Sticker {
        &self.select_animation
    }

    pub fn activate_animation(&self) -> &Sticker {
        &self.activate_animation
    }

    pub fn effect_animation(&self) -> &Sticker {
        &self.effect_animation
    }

    pub fn around_animation(&self) -> &Option<Sticker> {
        &self.around_animation
    }

    pub fn center_animation(&self) -> &Option<Sticker> {
        &self.center_animation
    }
}

#[doc(hidden)]
pub struct RTDReactionBuilder {
    inner: Reaction,
}

impl RTDReactionBuilder {
    pub fn build(&self) -> Reaction {
        self.inner.clone()
    }

    pub fn reaction<T: AsRef<str>>(&mut self, reaction: T) -> &mut Self {
        self.inner.reaction = reaction.as_ref().to_string();
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn is_active(&mut self, is_active: bool) -> &mut Self {
        self.inner.is_active = is_active;
        self
    }

    pub fn is_premium(&mut self, is_premium: bool) -> &mut Self {
        self.inner.is_premium = is_premium;
        self
    }

    pub fn static_icon<T: AsRef<Sticker>>(&mut self, static_icon: T) -> &mut Self {
        self.inner.static_icon = static_icon.as_ref().clone();
        self
    }

    pub fn appear_animation<T: AsRef<Sticker>>(&mut self, appear_animation: T) -> &mut Self {
        self.inner.appear_animation = appear_animation.as_ref().clone();
        self
    }

    pub fn select_animation<T: AsRef<Sticker>>(&mut self, select_animation: T) -> &mut Self {
        self.inner.select_animation = select_animation.as_ref().clone();
        self
    }

    pub fn activate_animation<T: AsRef<Sticker>>(&mut self, activate_animation: T) -> &mut Self {
        self.inner.activate_animation = activate_animation.as_ref().clone();
        self
    }

    pub fn effect_animation<T: AsRef<Sticker>>(&mut self, effect_animation: T) -> &mut Self {
        self.inner.effect_animation = effect_animation.as_ref().clone();
        self
    }

    pub fn around_animation<T: AsRef<Sticker>>(&mut self, around_animation: T) -> &mut Self {
        self.inner.around_animation = Some(around_animation.as_ref().clone());
        self
    }

    pub fn center_animation<T: AsRef<Sticker>>(&mut self, center_animation: T) -> &mut Self {
        self.inner.center_animation = Some(center_animation.as_ref().clone());
        self
    }
}

impl AsRef<Reaction> for Reaction {
    fn as_ref(&self) -> &Reaction {
        self
    }
}

impl AsRef<Reaction> for RTDReactionBuilder {
    fn as_ref(&self) -> &Reaction {
        &self.inner
    }
}
