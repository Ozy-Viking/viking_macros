use viking_macros_enum::EnumDisplay;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, EnumDisplay)]
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
fn enum_display_string() {
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
        assert_eq!(variant.to_string(), expected);
        assert_eq!(format!("{variant}"), expected);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, EnumDisplay)]
enum Unconverted {
    KeepPascal,
    HTTPRequest,
}

#[test]
fn enum_display_keeps_ident_without_case_attribute() {
    assert_eq!(Unconverted::KeepPascal.to_string(), "KeepPascal");
    assert_eq!(Unconverted::HTTPRequest.to_string(), "HTTPRequest");
}

#[derive(Debug, Clone, Copy, PartialEq, EnumDisplay)]
#[Snake]
enum Overridden {
    UsesEnumDefault,
    #[Kebab]
    VariantOverride,
    #[None]
    KeepExactName,
}

#[test]
fn enum_display_variant_attribute_overrides_enum_attribute() {
    assert_eq!(Overridden::UsesEnumDefault.to_string(), "uses_enum_default");
    assert_eq!(Overridden::VariantOverride.to_string(), "variant-override");
    assert_eq!(Overridden::KeepExactName.to_string(), "KeepExactName");
}

#[derive(Debug, Clone, Copy, PartialEq, EnumDisplay)]
#[None]
enum ExplicitNone {
    HTTPRequest,
    AlreadyCamel,
}

#[test]
fn enum_display_none_keeps_variant_idents() {
    assert_eq!(ExplicitNone::HTTPRequest.to_string(), "HTTPRequest");
    assert_eq!(ExplicitNone::AlreadyCamel.to_string(), "AlreadyCamel");
}

#[derive(Debug, Clone, Copy, PartialEq, EnumDisplay)]
#[Flat]
enum Single {
    OnlyVariant,
}

#[test]
fn enum_display_supports_a_single_variant() {
    assert_eq!(Single::OnlyVariant.to_string(), "onlyvariant");
}
