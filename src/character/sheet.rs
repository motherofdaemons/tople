use super::attributes::Attributes;

pub struct Sheet {
    name: String,
    class: Class,
    hp: i32,
    max_hp: i32,
    ac: i32,
    attrs: Attributes,
}

enum Class {
    Artificer,
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}