//!
//! # i18n
//!
//! Internationalization module. Uses Fluent for translations. If the targeted language is not
//! available, it will fallback to English.
//!
use std::{borrow::Cow, collections::HashMap};

use fluent_bundle::bundle::FluentBundle;
use fluent_bundle::{FluentArgs, FluentResource};
use intl_memoizer::concurrent;
use unic_langid::LanguageIdentifier;

pub struct Font {
    pub location: &'static str,
}

pub type Fonts = HashMap<String, Font>;

/// [`Language`] contains all in-game text translations and their fonts.
/// It is required to have following functions with good reasons:
/// - get a message by their key: so we can display it in different languages.
/// - load translations: of course
/// - hot reload when the translation file changes: convenient for development
/// - hot reload when language changes: should not quit-enter to change language, that's dumb.
struct Language {
    pub bundle: FluentBundle<FluentResource, concurrent::IntlLangMemoizer>,
    pub locale: LanguageIdentifier,
    pub fonts: Fonts,
}

impl assets_manager::Compound for Language {

    fn load(
        cache: assets_manager::AnyCache,
        id: &assets_manager::SharedString,
    ) -> Result<Self, assets_manager::BoxedError> {

        todo!()
    }

}

impl Language {
    pub fn try_get_message(&self, key: &str, args: Option<&FluentArgs>) -> Option<Cow<str>> {
        let msg = self.bundle.get_message(key)?;
        let mut errors = vec![];
        let ret = self.bundle.format_pattern(msg.value()?, args, &mut errors);
        match errors.is_empty() {
            true => Some(ret.into()),
            false => {
                tracing::error!("Failed to format pattern: {:?}", errors);
                None
            }
        }
    }

    #[cfg(test)]
    fn test_try_get_message() {}
}

struct i18nProvider {
    handle: assets_manager::Handle<'static, Language>,
    watcher: assets_manager::ReloadWatcher<'static>,
}

impl i18nProvider {
    pub fn load(lang: &str) -> Result<Self, anyhow::Error> {
        todo!()
    }
}
