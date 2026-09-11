use viking_macros_enum::EnumAsStr;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, EnumAsStr)]
#[Lower]
enum CaseTest {
    #[Snake]
    SnakeCase,
    #[Constant]
    ConstantCase,
    #[UpperSnake]
    UpperSnakeCase,
    #[Ada]
    AdaCase,
    #[Kebab]
    KebabCase,
    #[Cobol]
    CobolCase,
    #[UpperKebab]
    UpperKebabCase,
    #[Train]
    TrainCase,
    #[Flat]
    FlatCase,
    #[UpperFlat]
    UpperFlatCase,
    #[Pascal]
    PascalCase,
    #[UpperCamel]
    UpperCamelCase,
    #[Camel]
    CamelCase,
    #[Lower]
    LowerCase,
    #[Upper]
    UpperCase,
    #[Title]
    TitleCase,
    #[Sentence]
    SentenceCase,
    #[None]
    NoneCase,
    DefaultCase,
}

#[test]
fn enum_as_str_converts_each_variant() {
    let cases = [
        (CaseTest::SnakeCase, "snake_case"),
        (CaseTest::ConstantCase, "CONSTANT_CASE"),
        (CaseTest::UpperSnakeCase, "UPPER_SNAKE_CASE"),
        (CaseTest::AdaCase, "Ada_Case"),
        (CaseTest::KebabCase, "kebab-case"),
        (CaseTest::CobolCase, "COBOL-CASE"),
        (CaseTest::UpperKebabCase, "UPPER-KEBAB-CASE"),
        (CaseTest::TrainCase, "Train-Case"),
        (CaseTest::FlatCase, "flatcase"),
        (CaseTest::UpperFlatCase, "UPPERFLATCASE"),
        (CaseTest::PascalCase, "PascalCase"),
        (CaseTest::UpperCamelCase, "UpperCamelCase"),
        (CaseTest::CamelCase, "camelCase"),
        (CaseTest::LowerCase, "lower case"),
        (CaseTest::UpperCase, "UPPER CASE"),
        (CaseTest::TitleCase, "Title Case"),
        (CaseTest::SentenceCase, "Sentence case"),
        (CaseTest::NoneCase, "NoneCase"),
        (CaseTest::DefaultCase, "default case"),
    ];

    for (variant, expected) in cases {
        assert_eq!(variant.as_str(), expected);
    }
}

#[derive(Debug, Clone, Copy, EnumAsStr)]
#[Kebab]
enum Color {
    Primary,
    Secondary,
    ErrorBackground,
}

#[test]
fn enum_as_str_applies_enum_level_case() {
    assert_eq!(Color::Primary.as_str(), "primary");
    assert_eq!(Color::Secondary.as_str(), "secondary");
    assert_eq!(Color::ErrorBackground.as_str(), "error-background");
}

#[test]
fn enum_as_str_is_usable_in_const() {
    const PRIMARY: &str = Color::Primary.as_str();
    const SECONDARY: &str = Color::Secondary.as_str();
    const ERROR_BACKGROUND: &str = Color::ErrorBackground.as_str();

    assert_eq!(PRIMARY, "primary");
    assert_eq!(SECONDARY, "secondary");
    assert_eq!(ERROR_BACKGROUND, "error-background");
}

#[derive(Debug, Clone, Copy, EnumAsStr)]
enum Unconverted {
    KeepPascal,
    HTTPRequest,
}

#[test]
fn enum_as_str_keeps_ident_without_case_attribute() {
    assert_eq!(Unconverted::KeepPascal.as_str(), "KeepPascal");
    assert_eq!(Unconverted::HTTPRequest.as_str(), "HTTPRequest");
}

#[derive(Debug, Clone, Copy, EnumAsStr)]
#[Snake]
enum Overridden {
    UsesEnumDefault,
    #[Kebab]
    VariantOverride,
    #[None]
    KeepExactName,
}

#[test]
fn enum_as_str_variant_attribute_overrides_enum_attribute() {
    assert_eq!(Overridden::UsesEnumDefault.as_str(), "uses_enum_default");
    assert_eq!(Overridden::VariantOverride.as_str(), "variant-override");
    assert_eq!(Overridden::KeepExactName.as_str(), "KeepExactName");
}

#[derive(Debug, Clone, Copy, EnumAsStr)]
#[None]
enum ExplicitNone {
    HTTPRequest,
    AlreadyCamel,
}

#[test]
fn enum_as_str_none_keeps_variant_idents() {
    assert_eq!(ExplicitNone::HTTPRequest.as_str(), "HTTPRequest");
    assert_eq!(ExplicitNone::AlreadyCamel.as_str(), "AlreadyCamel");
}

#[derive(Debug, Clone, Copy, EnumAsStr)]
#[Flat]
enum Single {
    OnlyVariant,
}

#[test]
fn enum_as_str_supports_a_single_variant() {
    assert_eq!(Single::OnlyVariant.as_str(), "onlyvariant");
}

const fn const_as_str(color: Color) -> &'static str {
    color.as_str()
}

#[test]
fn enum_as_str_can_be_called_from_a_const_fn() {
    const VALUE: &str = const_as_str(Color::ErrorBackground);
    assert_eq!(VALUE, "error-background");
}
