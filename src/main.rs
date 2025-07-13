use viking_macros::EnumVec;

fn main() {
    println!("{:?}", Test::all_variants())
}

#[derive(Debug, EnumVec)]
enum Test {
    Compleded,
    NoTested,
}
