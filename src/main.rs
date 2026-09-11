#![allow(dead_code)]
use viking_macros::EnumAsStr;

fn main() {
    println!("{}", Test::RosePine.as_str())
}

#[derive(Debug, EnumAsStr)]
#[Kebab]
enum Test {
    Catppuccin,
    TokyoNight,
    RosePine,
}
