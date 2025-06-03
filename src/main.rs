use viking_macros::EnumDisplay;

fn main() {
    println!("{}", Work::Yes);
}

#[derive(EnumDisplay)]
#[uppercase]
enum Work {
    Yes,
    No,
}
