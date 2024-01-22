use std::collections::HashMap;

use crate::{
    data::{
        repository::{script::ScriptRepository, text::TextRepository},
        source::{language::LanguageFileDataSource, script::ScriptFileDataSource},
    },
    gameplay::{
        class::{ClassTree, ClassTreeDescription},
        script::Director,
    },
    setting::Setting,
};
use tracing::info;
use unic_langid::LanguageIdentifier;

/// [`GameState`] stores states that shared by whole game.
///
/// It is layered as below:
///
/// 1. Connection layer
///     This layer connects the game backend to UI layer, provides an event API for
/// them to communicate. The backend does not need to know how UI display the game,
///
/// 2. Gameplay layer
///     This layer deals with game logic, processes data and give them to the UI.
///
/// 3. Data layer
///     This layer provides the game data from various data sources like local assets,
/// mods or the Internet.
pub struct GameState {
    setting: Setting,
    class_tree: ClassTree,
    pub text: TextRepository,
    director: Director,
}

impl Default for GameState {
    fn default() -> Self {
        let setting = match Setting::load(Setting::DEFAULT_SETTINGS_PATH) {
            Ok(setting) => {
                info!("find user setting {:?}", setting);
                setting
            }
            Err(err) => {
                info!("Did not find user setting, using default: {}", err);
                Setting::default()
            }
        };
        setting.save().expect("setting configuration cannot save");
        let text_source = LanguageFileDataSource::new();
        let script_source = ScriptFileDataSource::new();
        let mut text = TextRepository::new(text_source);
        let script_repo = ScriptRepository::new(script_source);

        let director = Director::new(script_repo, &setting.gameplay.entry_point);
        text.load(setting.language.language.parse().unwrap());
        Self {
            setting,
            class_tree: ClassTree::default(),
            text,
            director,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    ChangeLanguage(LanguageIdentifier),
}

impl GameState {
    pub fn get_skill_tree(&self) -> (&ClassTree, HashMap<String, ClassTreeDescription>) {
        (
            &self.class_tree,
            self.class_tree.get_descriptions(&self.text),
        )
    }

    pub fn get_next_line(&mut self) -> String {
        self.director.next_line(&self.text)
    }

    pub fn command_handler(&mut self, command: Command) {
        match command {
            Command::ChangeLanguage(lang) => {
                self.setting.language.language = lang.to_string();
                self.setting.save().unwrap();
                self.text.load(lang);
            }
        }
    }
}
