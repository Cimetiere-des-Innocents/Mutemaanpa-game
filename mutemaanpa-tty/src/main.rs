use mutemaanpa_lib::{
    class::ClassNode,
    game_state::{run, GameState},
    i18n::I18nProvider,
};
use tracing::info;

fn main() {
    tracing_subscriber::fmt::init();
    info!("tty client started");
    let game_state = GameState::default();
    print_class_tree(&game_state);
    run(game_state);
    info!("loop forever");
    loop {}
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
