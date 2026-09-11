use viking_macros_enum::{EnumAsStr, EnumDisplay, EnumVec};

#[derive(Debug, Clone, Copy, PartialEq, EnumVec)]
enum Status {
    Ready,
    Running,
    Done,
}

#[test]
fn all_variants_returns_each_variant_in_declaration_order() {
    assert_eq!(
        Status::all_variants(),
        vec![Status::Ready, Status::Running, Status::Done]
    );
}

#[test]
fn all_variants_contains_each_variant_once() {
    let variants = Status::all_variants();
    assert_eq!(variants.len(), 3);
    assert!(variants.contains(&Status::Ready));
    assert!(variants.contains(&Status::Running));
    assert!(variants.contains(&Status::Done));
}

#[derive(Debug, Clone, Copy, PartialEq, EnumVec)]
enum Single {
    Only,
}

#[test]
fn all_variants_supports_a_single_variant() {
    assert_eq!(Single::all_variants(), vec![Single::Only]);
}

#[derive(Debug, Clone, Copy, PartialEq, EnumVec)]
enum Never {}

#[test]
fn all_variants_is_empty_for_an_enum_with_no_variants() {
    assert_eq!(Never::all_variants(), Vec::<Never>::new());
}

#[derive(Debug, Clone, Copy, PartialEq, EnumDisplay, EnumAsStr, EnumVec)]
#[Kebab]
enum Theme {
    Catppuccin,
    TokyoNight,
    RosePine,
}

#[test]
fn enum_vec_composes_with_display_and_as_str() {
    assert_eq!(
        Theme::all_variants(),
        vec![Theme::Catppuccin, Theme::TokyoNight, Theme::RosePine]
    );
    assert_eq!(Theme::RosePine.to_string(), "rose-pine");
    assert_eq!(Theme::RosePine.as_str(), "rose-pine");
    assert_eq!(Theme::TokyoNight.as_str(), "tokyo-night");
}
