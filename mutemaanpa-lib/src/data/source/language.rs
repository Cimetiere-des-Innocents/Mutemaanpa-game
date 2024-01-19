//! # Language data sources
//!

use std::collections::HashMap;

use anyhow::Result;
use assets_manager::{loader::StringLoader, Handle};
use fluent_bundle::{bundle::FluentBundle, FluentResource};
use intl_memoizer::concurrent;
use serde::{Deserialize, Serialize};
use unic_langid::LanguageIdentifier;

use super::ASSETS;

/// [`LanguagePack`] contains three things:
///     1. Which language? This is provided by [`LanguageIdentifier`].
///     2. What this line translates in target language? This is provided by [`FluentBundle`].
///     3. What font should we use in this language? This is provided by [`Fonts`]
///
/// This is a plain data struct, which is provided by [`LanguageHotFileDataSource`]. It is then
pub struct LanguagePack {
    pub locale: LanguageIdentifier,
    pub bundle: FluentBundle<FluentResource, concurrent::IntlLangMemoizer>,
    pub fonts: Fonts,
}

/// Language Packs are dynamic assets, so we load them using asset_manager.
impl assets_manager::Compound for LanguagePack {
    fn load(
        cache: assets_manager::AnyCache,
        id: &assets_manager::SharedString,
    ) -> Result<Self, assets_manager::BoxedError> {
        let locale: LanguageIdentifier = id.parse()?;
        let manifest = cache.load::<LanguageManifest>(&["language.", id, ".manifest"].concat())?;

        let mut bundle = FluentBundle::new_concurrent(vec![locale.clone()]);
        cache
            .load_rec_dir::<FluentFilePath>(&["language.", id].concat())?
            .read()
            .ids()
            .flat_map(|id| load_ftl(&cache, id))
            .map(|lang_file| FluentResource::try_new(lang_file).unwrap())
            .for_each(|lang_res| bundle.add_resource(lang_res).unwrap());

        Ok(Self {
            bundle,
            locale,
            fonts: manifest.read().fonts.clone(),
        })
    }
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

#[test]
fn test_load_language_pack() {
    use crate::data::source::ASSETS;
    crate::tests_utils::logging_init();
    let lang = ASSETS.load::<LanguagePack>("en").unwrap();
    assert_eq!(
        lang.read().locale,
        "en".parse::<LanguageIdentifier>().unwrap()
    );
    assert!(lang.read().bundle.get_message("Cleric").is_some());
}

/// [`LanguageManifest`] is the language-pack level metadata, containing the fonts
/// and language identifier. It is used to load/store directly so its fields are public and plain.
/// Because we can't serialize [`LanguageIdentifier`], so this struct
/// is a bridge.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct LanguageManifest {
    pub fonts: Fonts,
    pub language: String,
}

/// [`Fonts`] contains all fonts used in a set of translation.
/// We are often in need of multiple fonts to achieve good display effect.
/// At least there are sans-serif and serif fonts.
pub type Fonts = HashMap<String, Font>;

/// [`Font`] contains the location of a font file.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct Font {
    pub location: String,
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

/// Language manifest is loaded by [`assets_manager`].
impl assets_manager::Asset for LanguageManifest {
    const EXTENSION: &'static str = "yaml";
    type Loader = assets_manager::loader::YamlLoader;
}

pub struct LanguageFileDataSource;

impl LanguageFileDataSource {
    pub fn new() -> LanguageFileDataSource {
        LanguageFileDataSource
    }

    pub fn get_language_pack(&self, lang: LanguageIdentifier) -> &'static Handle<LanguagePack> {
        ASSETS.load(&lang.to_string()).unwrap()
    }
}

impl Default for LanguageFileDataSource {
    fn default() -> Self {
        Self::new()
    }
}
