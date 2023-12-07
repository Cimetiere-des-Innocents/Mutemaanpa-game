//!
//! # i18n
//!
//! Internationalization module. Uses Fluent for translations. If the targeted language is not
//! available, it will fallback to English.
//!
use std::{borrow::Cow, collections::HashMap};

use assets_manager::loader::StringLoader;
use fluent_bundle::bundle::FluentBundle;
use fluent_bundle::{FluentArgs, FluentResource};
use intl_memoizer::concurrent;
use serde::{Deserialize, Serialize};
use unic_langid::LanguageIdentifier;

/// This trait is used, but for now the rust-analyzer cannot detect it.
#[allow(unused_imports)]
use crate::loader::GameAsset;
#[warn(unused_imports)]

/// [`Font`] contains the location of a font file.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Font {
    pub location: String,
}

/// [`Fonts`] contains all fonts used in a set of translation.
/// We are often in need of multiple fonts to achieve good display effect.
/// At least there are sans-serif and serif fonts.
pub type Fonts = HashMap<String, Font>;

/// [`LanguageManifest`] is the language-pack level metadata, containing the fonts
/// and language identifier. It is used to load/store directly so its fields are public and plain.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct LanguageManifest {
    pub fonts: Fonts,
    pub language: String,
}

#[test]
fn test_manifest() {
    crate::tests_utils::logging_init();
    let manifest = LanguageManifest {
        fonts: {
            let mut map = HashMap::new();
            map.insert(
                String::from("regular"),
                Font {
                    location: String::from("font.ttf"),
                },
            );
            map.insert(
                String::from("serif"),
                Font {
                    location: String::from("font2.ttf"),
                },
            );
            map
        },
        language: "en-US".to_string(),
    };
    let manifest_str = serde_yaml::to_string(&manifest);
    let recovered_struct: LanguageManifest = serde_yaml::from_str(&manifest_str.unwrap()).unwrap();
    assert_eq!(manifest, recovered_struct);
}

/// Not unreasonable to require it be loaded by [`assets_manager`].
impl assets_manager::Asset for LanguageManifest {
    const EXTENSION: &'static str = "yaml";
    type Loader = assets_manager::loader::YamlLoader;
}

/// [`Language`] contains all in-game text translations and their fonts.
/// It is required to have following functions with good reasons:
/// - get a message by their key: so we can display it in different languages.
/// - load translations: of course
/// - hot reload when the translation file changes: convenient for development
/// - hot reload when language changes: should not quit-enter to change language, that's dumb.
pub struct Language {
    pub bundle: FluentBundle<FluentResource, concurrent::IntlLangMemoizer>,
    pub locale: LanguageIdentifier,
    pub fonts: Fonts,
}

struct FluentFilePath(String);

impl From<String> for FluentFilePath {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl assets_manager::Asset for FluentFilePath {
    const EXTENSION: &'static str = "ftl";
    type Loader = assets_manager::loader::LoadFrom<String, StringLoader>;
}

fn load_ftl(cache: &assets_manager::AnyCache, id: &assets_manager::SharedString) -> Option<String> {
    match cache.load::<FluentFilePath>(id) {
        Ok(lang_file) => {
            tracing::info!("Loaded language file: {}", id);
            Some(lang_file.read().0.clone())
        }
        Err(e) => {
            tracing::error!("Failed to load language file: {}", e);
            None
        }
    }
}

impl assets_manager::Compound for Language {
    fn load(
        cache: assets_manager::AnyCache,
        id: &assets_manager::SharedString,
    ) -> Result<Self, assets_manager::BoxedError> {
        let lang_id: LanguageIdentifier = id.parse()?;
        let manifest = cache.load::<LanguageManifest>(&["language.", id, ".manifest"].concat())?;

        let mut bundle = FluentBundle::new_concurrent(vec![lang_id.clone()]);
        cache
            .load_dir::<FluentFilePath>(&["language.", id].concat(), true)?
            .ids()
            .map(|id| load_ftl(&cache, id))
            .flatten()
            .map(|lang_file| FluentResource::try_new(lang_file).unwrap())
            .for_each(|lang_res| bundle.add_resource(lang_res).unwrap());

        Ok(Self {
            bundle,
            locale: lang_id,
            fonts: manifest.read().fonts.clone(),
        })
    }
}

#[test]
fn test_load_language_pack() {
    crate::tests_utils::logging_init();
    let lang = Language::load("en").unwrap();
    assert_eq!(
        lang.read().locale,
        "en".parse::<LanguageIdentifier>().unwrap()
    );
    assert!(lang.read().bundle.get_message("Cleric").is_some());
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

    pub fn try_get_attr(
        &self,
        key: &str,
        attr: &str,
        args: Option<&FluentArgs>,
    ) -> Option<Cow<str>> {
        let msg = self.bundle.get_message(key)?;
        let mut errors = vec![];
        let ret = self
            .bundle
            .format_pattern(msg.get_attribute(attr)?.value(), args, &mut errors);
        match errors.is_empty() {
            true => Some(ret.into()),
            false => {
                tracing::error!("Failed to format pattern: {:?}", errors);
                None
            }
        }
    }
}

#[test]
fn test_try_get_message() {
    crate::tests_utils::logging_init();
    let lang = Language::load("en").unwrap();
    assert_eq!(
        lang.read().try_get_message("Cleric", None),
        Some(Cow::from("Cleric"))
    );
}

/// [`i18nProvider`] is the main interface of this module.
/// It is used to get translated messages and fonts. The asset is given by value. We can't pass reference to ASSETS,
/// because ASSETS can be updated by hot-reloading, which acquires a write lock, conflicting with the read lock.
pub struct I18nProvider {
    handle: assets_manager::Handle<'static, Language>,
    watcher: assets_manager::ReloadWatcher<'static>,
}

impl I18nProvider {
    const MISSING_MSG: &'static str = "MISSING";

    pub fn load(lang: &str) -> Result<Self, anyhow::Error> {
        let handle = Language::load(lang)?;
        Ok(Self {
            handle,
            watcher: handle.reload_watcher(),
        })
    }

    pub fn reloaded(&mut self) -> bool {
        self.watcher.reloaded()
    }

    fn try_get_msg(&self, key: &str, args: Option<&FluentArgs>) -> Option<String> {
        self.handle
            .read()
            .try_get_message(key, args)
            .map(|s| s.into_owned())
    }

    pub fn get_msg_or_default(&self, key: &str, args: Option<&FluentArgs>) -> String {
        self.try_get_msg(key, args)
            .unwrap_or_else(|| Self::MISSING_MSG.into())
    }

    fn get_attr(&self, key: &str, attr: &str, args: Option<&FluentArgs>) -> Option<String> {
        self.handle
            .read()
            .try_get_attr(key, attr, args)
            .map(|s| s.into_owned())
    }

    pub fn get_attr_or_default(&self, key: &str, attr: &str, args: Option<&FluentArgs>) -> String {
        self.get_attr(key, attr, args)
            .unwrap_or_else(|| Self::MISSING_MSG.into())
    }
}

impl Default for I18nProvider {
    fn default() -> Self {
        Self::load("en").unwrap()
    }
}
