#![allow(dead_code)]
use super::attributes::Attributes;
use super::class::Class;

pub struct Sheet {
    name: String,
    class: Class,
    level: u8,
    hp: i32,
    max_hp: i32,
    attrs: Attributes,
}
