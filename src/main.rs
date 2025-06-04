use viking_macros::EnumDisplay;

fn main() {
    println!("{}", Work::YesNo);
    println!("{}", Work::NoYes);
}

#[derive(EnumDisplay)]
#[Cobol]
enum Work {
    #[Upper]
    YesNo,
    #[Ada]
    NoYes,
}
