use std::io::{stdin, stdout, Write};

use anyhow::anyhow;
use anyhow::Result;
use mutemaanpa_lib::{
    class::ClassNode,
    game_state::{run, GameState},
    i18n::{self, I18nProvider},
};
use tracing::info;

#[derive(Debug, Clone)]
enum Command {
    ChangeLanguage(String),
}

fn main() {
    tracing_subscriber::fmt::init();
    info!("Game client settled.");
    let mut game_state = GameState::default();
    print_class_tree(&game_state);
    run(&mut game_state);
    info!("Starting main game loop");
    loop {
        render(&mut game_state);
        handle_user_input(&mut game_state);
        update(&mut game_state);
    }
}

// .......................... Render ..........................................

fn render(game_state: &mut GameState) {
    if game_state.i18n.reloaded() {
        info!("i18n reloaded");
        print!("\x1B[2J\x1B[1;1H");
        print_class_tree(&game_state);
    }
    prompt();
    stdout().flush().unwrap();
}

fn prompt() {
    print!("]=> ");
}

fn print_class_tree(game_state: &GameState) {
    let tree = game_state.get_skill_tree();
    fn print_class_node(node: &ClassNode, depth: usize, language: &I18nProvider) {
        let indent = " ".repeat(depth * 2);
        println!(
            "{}{}:\n\t {}",
            indent,
            language.get_msg_or_default(node.name, None),
            language.get_attr_or_default(node.name, "desc", None),
        );
        for child in &node.children {
            print_class_node(&child, depth + 1, language);
        }
    }
    print_class_node(&tree.root, 0, &game_state.i18n)
}

// ................................. Control ..................................

fn handle_user_input(game_state: &mut GameState) {
    let user_input = get_user_input();
    info!("received user input: {}", user_input);
    let cmd = parse_user_input(user_input);
    match cmd {
        Err(e) => {
            info!("Invalid user input {}", e)
        }
        Ok(cmd) => {
            execute_cmd(cmd, game_state);
        }
    }
}

fn get_user_input() -> String {
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap();
    user_input
}

fn parse_user_input(s: String) -> Result<Command> {
    let mut tokens = s.split_whitespace();
    match tokens.next() {
        Some("change-language") => Ok(Command::ChangeLanguage(
            tokens
                .next()
                .ok_or(anyhow!("Change Language needs an argument: <language>"))?
                .to_string(),
        )),
        Some(cmd) => {
            info!("Unrecognized user input: {}", cmd);
            Err(anyhow!("Unrecognized user input: {}", cmd))
        }
        None => {
            info!("Empty input");
            Err(anyhow!("User didn't input anything."))
        }
    }
}

fn execute_cmd(cmd: Command, game_state: &mut GameState) {
    match cmd {
        Command::ChangeLanguage(lang) => match i18n::I18nProvider::load(&lang) {
            Ok(new_provider) => game_state.i18n = new_provider,
            Err(e) => info!(
                "Can't find the i18n support for language {}.\n\t Caused by {}",
                lang, e
            ),
        },
    }
}

// ................................. Update ...................................

fn update(_: &mut GameState) {}
