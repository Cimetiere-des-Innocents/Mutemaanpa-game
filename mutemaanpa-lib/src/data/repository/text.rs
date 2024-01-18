//!
//! # text repository
//!
//! Text module. It provides the game all user visible texts. Uses Fluent for translations.
//! It will always load english language pack.
//!
use std::{collections::HashMap, str::FromStr};

use assets_manager::{Handle, ReloadWatcher};
use fluent_bundle::FluentArgs;
use once_cell::sync::Lazy;
use tracing::warn;
use unic_langid::LanguageIdentifier;

use crate::data::source::language::{LanguageFileDataSource, LanguagePack};

/// [`TextRepository`] is the main interface of this module.
pub struct TextRepository {
    source: LanguageFileDataSource,
    handles: I18nHandles,
    watchers: I18nWatchers,
    first_language: LanguageIdentifier,
}

type I18nHandles = HashMap<LanguageIdentifier, Handle<'static, LanguagePack>>;
type I18nWatchers = HashMap<LanguageIdentifier, ReloadWatcher<'static>>;

static ENGLISH: Lazy<LanguageIdentifier> =
    Lazy::new(|| LanguageIdentifier::from_str("en").unwrap());

impl TextRepository {
    const MISSING_MSG: &'static str = "MISSING";

    /// Create a new [`TextRepository`] from [`LanguageHotFileDataSource`] with a english language pack.
    ///
    /// examples:
    /// ```rust
    /// use mutemaanpa_lib::data::source::language::LanguageFileDataSource;
    /// use mutemaanpa_lib::data::repository::text::TextRepository;
    /// let source = LanguageFileDataSource::new();
    /// let text_repo = TextRepository::new(source);
    /// ```
    pub fn new(source: LanguageFileDataSource) -> TextRepository {
        let mut text_repo = TextRepository {
            source,
            handles: HashMap::new(),
            watchers: HashMap::new(),
            first_language: ENGLISH.clone(),
        };
        text_repo.load(ENGLISH.clone());
        text_repo
    }

    pub fn load(&mut self, lang: LanguageIdentifier) {
        let handle = self.source.get_language_pack(lang.clone());
        let watcher = handle.reload_watcher();
        self.handles.insert(lang.clone(), handle);
        self.watchers.insert(lang.clone(), watcher);
        self.first_language = lang.clone();
    }

    /// This functions checks whether the language pack has updated since the last call
    /// to itself.
    /// Well such features may be better implemented using coroutines but for now just it.
    pub fn has_reloaded(&mut self, lang: LanguageIdentifier) -> bool {
        self.watchers
            .get_mut(&lang)
            .map(|x| x.reloaded())
            .unwrap_or(true)
    }

    pub fn get_message(&self, key: &str, args: Option<&FluentArgs>) -> String {
        self.get_message_fallback(key, args)
            .unwrap_or(Self::MISSING_MSG.to_string())
    }

    fn get_message_fallback(&self, key: &str, args: Option<&FluentArgs>) -> Option<String> {
        let bundle = &self
            .handles
            .get(&self.first_language)
            .unwrap_or(self.handles.get(&ENGLISH).unwrap())
            .read()
            .bundle;
        let pattern = bundle.get_message(key)?;
        let mut errors = vec![];
        let message = bundle.format_pattern(pattern.value()?, args, &mut errors);
        match errors.is_empty() {
            true => Some(message.into_owned()),
            false => {
                warn!("get message {} failed: {:?}", key, errors);
                None
            }
        }
    }

    pub fn get_attr(&self, key: &str, attr: &str, args: Option<&FluentArgs>) -> String {
        self.get_attr_fallback(key, attr, args)
            .unwrap_or(Self::MISSING_MSG.to_string())
    }

    fn get_attr_fallback(
        &self,
        key: &str,
        attr: &str,
        args: Option<&FluentArgs>,
    ) -> Option<String> {
        let bundle = &self
            .handles
            .get(&self.first_language)
            .unwrap_or(self.handles.get(&ENGLISH).unwrap())
            .read()
            .bundle;
        let pattern = bundle.get_message(key)?.get_attribute(attr)?.value();
        let mut errors = vec![];
        let message = bundle.format_pattern(pattern, args, &mut errors);
        match errors.is_empty() {
            true => Some(message.into_owned()),
            false => {
                warn!(
                    "get message {} with attribute {} failed: {:?}",
                    key, attr, errors
                );
                None
            }
        }
    }
}
