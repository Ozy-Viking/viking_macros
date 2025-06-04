use viking_macros_enum::EnumDisplay;

#[allow(clippy::enum_variant_names)]
#[derive(EnumDisplay)]
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
    #[Sentence()]
    SentenceCase,
    #[Alternating()]
    AlternatingCase,
    #[Toggle()]
    ToggleCase,
    #[None]
    NoneCase,
    DefaultCase,
}

#[test]
fn enum_display_string() {
    assert_eq!(CaseTest::SnakeCase.to_string().as_str(), "snake_case");
    assert_eq!(CaseTest::ConstantCase.to_string().as_str(), "CONSTANT_CASE");
    assert_eq!(
        CaseTest::UpperSnakeCase.to_string().as_str(),
        "UPPER_SNAKE_CASE"
    );
    assert_eq!(CaseTest::AdaCase.to_string().as_str(), "Ada_Case");
    assert_eq!(CaseTest::KebabCase.to_string().as_str(), "kebab-case");
    assert_eq!(CaseTest::CobolCase.to_string().as_str(), "COBOL-CASE");
    assert_eq!(
        CaseTest::UpperKebabCase.to_string().as_str(),
        "UPPER-KEBAB-CASE"
    );
    assert_eq!(CaseTest::TrainCase.to_string().as_str(), "Train-Case");
    assert_eq!(CaseTest::FlatCase.to_string().as_str(), "flatcase");
    assert_eq!(
        CaseTest::UpperFlatCase.to_string().as_str(),
        "UPPERFLATCASE"
    );
    assert_eq!(CaseTest::PascalCase.to_string().as_str(), "PascalCase");
    assert_eq!(
        CaseTest::UpperCamelCase.to_string().as_str(),
        "UpperCamelCase"
    );
    assert_eq!(CaseTest::CamelCase.to_string().as_str(), "camelCase");
    assert_eq!(CaseTest::LowerCase.to_string().as_str(), "lower case");
    assert_eq!(CaseTest::UpperCase.to_string().as_str(), "UPPER CASE");
    assert_eq!(CaseTest::TitleCase.to_string().as_str(), "Title Case");
    assert_eq!(CaseTest::SentenceCase.to_string().as_str(), "Sentence case");
    assert_eq!(
        CaseTest::AlternatingCase.to_string().as_str(),
        "aLtErNaTiNg CaSe"
    );
    assert_eq!(CaseTest::ToggleCase.to_string().as_str(), "tOGGLE cASE");
    assert_eq!(CaseTest::NoneCase.to_string().as_str(), "NoneCase");
    assert_eq!(CaseTest::DefaultCase.to_string().as_str(), "default case");
}
