type ClassIdentifier = &'static str;

/// # Classes
///
/// Every player can choose to upgrade their classes when level up.
/// Classes are modelled after a tree, with the root being the base class.
/// Classes have subclasses. They are children of their parent class.
/// Classes can have multiple children, but only one parent.
/// Classes give player accesses to new abilities and skills.
#[derive(Debug, Clone)]
pub struct ClassNode {
    pub name: ClassIdentifier,
    pub children: Vec<ClassNode>,
}

fn class_leaf(name: ClassIdentifier) -> ClassNode {
    ClassNode {
        name,
        children: vec![],
    }
}

fn default_class_node() -> ClassNode {
    ClassNode {
        name: "Base",
        children: default_main_classes(),
    }
}

fn default_main_classes() -> Vec<ClassNode> {
    vec![
        default_citizen(),
        default_fighter(),
        default_scholar(),
        default_cleric(),
    ]
}

fn default_citizen() -> ClassNode {
    ClassNode {
        name: "Citizen",
        children: vec![
            class_leaf("Rogue"),
            class_leaf("Merchant"),
            class_leaf("Craftsperson"),
        ],
    }
}

fn default_fighter() -> ClassNode {
    ClassNode {
        name: "Fighter",
        children: vec![
            class_leaf("Berserker"),
            class_leaf("Paladin"),
            class_leaf("Ranger"),
        ],
    }
}

fn default_scholar() -> ClassNode {
    ClassNode {
        name: "Scholar",
        children: vec![
            class_leaf("Wizard"),
            class_leaf("Mechanic"),
            class_leaf("Alchemist"),
        ],
    }
}

fn default_cleric() -> ClassNode {
    ClassNode {
        name: "Cleric",
        children: vec![
            class_leaf("Priest"),
            class_leaf("Bard"),
            class_leaf("Gestalt"),
        ],
    }
}

#[test]
fn test_show_default_classes() {
    let classes = default_main_classes();
    println!("{:#?}", classes);
}

#[derive(Debug, Clone)]
pub struct ClassTree {
    pub root: ClassNode
}

impl Default for ClassTree {
    fn default() -> Self {
        Self {
            root: default_class_node()
        }
    }
}

