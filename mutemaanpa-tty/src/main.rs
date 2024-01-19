use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::process::exit;

use anyhow::anyhow;
use anyhow::Result;
use mutemaanpa_lib::{
    game_state::GameState, gameplay::class::ClassNode, gameplay::class::ClassTreeDescription,
};
use tracing::info;

#[derive(Debug, Clone)]
enum Command {
    ChangeLanguage(String),
    PrintClasses,
    QuitGame,
}

fn main() {
    tracing_subscriber::fmt::init();
    info!("Game client settled.");
    let mut game_state = GameState::default();
    info!("Starting main game loop");
    loop {
        render(&mut game_state);
        handle_user_input(&mut game_state);
        update(&mut game_state);
    }
}

// .......................... Render ..........................................

fn render(_: &mut GameState) {
    prompt();
    stdout().flush().unwrap();
}

fn prompt() {
    print!("]=> ");
}

fn print_class_tree(game_state: &GameState) {
    let (tree, tree_text) = game_state.get_skill_tree();
    fn print_class_node(
        node: &ClassNode,
        depth: usize,
        tree_text: &HashMap<String, ClassTreeDescription>,
    ) {
        let indent = " ".repeat(depth * 2);
        let ClassTreeDescription { name, detail } = tree_text.get(node.name).unwrap();
        println!("{}{}:\n\t {}", indent, name, detail,);
        for child in &node.children {
            print_class_node(child, depth + 1, tree_text);
        }
    }
    print_class_node(&tree.root, 0, &tree_text)
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
        Some("print-classes") => Ok(Command::PrintClasses),
        Some("quit-game") => Ok(Command::QuitGame),
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

const EXIT_SUCCESS: i32 = 0;

fn execute_cmd(cmd: Command, game_state: &mut GameState) {
    match cmd {
        Command::ChangeLanguage(lang) => game_state.command_handler(
            mutemaanpa_lib::game_state::Command::ChangeLanguage(lang.parse().unwrap()),
        ),
        Command::PrintClasses => print_class_tree(game_state),
        Command::QuitGame => {
            println!("We bid you farewell!");
            exit(EXIT_SUCCESS)
        }
    }
}

// ................................. Update ...................................

fn update(_: &mut GameState) {}
