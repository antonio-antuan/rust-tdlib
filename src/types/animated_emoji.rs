use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes an animated representation of an emoji
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AnimatedEmoji {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Animated sticker for the emoji
    sticker: Sticker,
    /// Emoji modifier fitzpatrick type; 0-6; 0 if none

    #[serde(default)]
    fitzpatrick_type: i32,
    /// File containing the sound to be played when the animated emoji is clicked if any; may be null. The sound is encoded with the Opus codec, and stored inside an OGG container
    sound: Option<File>,
}

impl RObject for AnimatedEmoji {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl AnimatedEmoji {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AnimatedEmojiBuilder {
        let mut inner = AnimatedEmoji::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        AnimatedEmojiBuilder { inner }
    }

    pub fn sticker(&self) -> &Sticker {
        &self.sticker
    }

    pub fn fitzpatrick_type(&self) -> i32 {
        self.fitzpatrick_type
    }

    pub fn sound(&self) -> &Option<File> {
        &self.sound
    }
}

#[doc(hidden)]
pub struct AnimatedEmojiBuilder {
    inner: AnimatedEmoji,
}

#[deprecated]
pub type RTDAnimatedEmojiBuilder = AnimatedEmojiBuilder;

impl AnimatedEmojiBuilder {
    pub fn build(&self) -> AnimatedEmoji {
        self.inner.clone()
    }

    pub fn sticker<T: AsRef<Sticker>>(&mut self, sticker: T) -> &mut Self {
        self.inner.sticker = sticker.as_ref().clone();
        self
    }

    pub fn fitzpatrick_type(&mut self, fitzpatrick_type: i32) -> &mut Self {
        self.inner.fitzpatrick_type = fitzpatrick_type;
        self
    }

    pub fn sound<T: AsRef<File>>(&mut self, sound: T) -> &mut Self {
        self.inner.sound = Some(sound.as_ref().clone());
        self
    }
}

impl AsRef<AnimatedEmoji> for AnimatedEmoji {
    fn as_ref(&self) -> &AnimatedEmoji {
        self
    }
}

impl AsRef<AnimatedEmoji> for AnimatedEmojiBuilder {
    fn as_ref(&self) -> &AnimatedEmoji {
        &self.inner
    }
}
