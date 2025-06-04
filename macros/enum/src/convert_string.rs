use proc_macro2::{Ident, Span};
use quote::{ToTokens, TokenStreamExt};
use std::str::FromStr;
use syn::Attribute;

/// See [Case] for definitions of CaseTypes
///
/// | pattern | underscore `_` | hyphen `-` | empty string | space |
/// | ---: | --- | --- | --- | --- |
/// | [lowercase](pattern::lowercase) | [snake_case](Case::Snake) | [kebab-case](Case::Kebab) | [flatcase](Case::Flat) | [lower case](Case::Lower) |
/// | [uppercase](pattern::uppercase) | [CONSTANT_CASE](Case::Constant) | [COBOL-CASE](Case::Cobol) | [UPPERFLATCASE](Case::UpperFlat) | [UPPER CASE](Case::Upper) |
/// | [capital](pattern::capital) | [Ada_Case](Case::Ada) | [Train-Case](Case::Train) | [PascalCase](Case::Pascal) | [Title Case](Case::Title) |
/// | [camel](pattern::camel) | | | [camelCase](Case::Camel) |
///
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum CaseType {
    Snake,
    Constant,
    UpperSnake,
    Ada,
    Kebab,
    Cobol,
    UpperKebab,
    Train,
    Flat,
    UpperFlat,
    Pascal,
    UpperCamel,
    Camel,
    Lower,
    Upper,
    Title,
    Sentence,
    Alternating,
    Toggle,
    #[default]
    None,
}

impl CaseType {
    pub fn all_cases() -> [CaseType; 20] {
        use CaseType as CT;
        [
            CT::Snake,
            CT::Constant,
            CT::UpperSnake,
            CT::Ada,
            CT::Kebab,
            CT::Cobol,
            CT::UpperKebab,
            CT::Train,
            CT::Flat,
            CT::UpperFlat,
            CT::Pascal,
            CT::UpperCamel,
            CT::Camel,
            CT::Lower,
            CT::Upper,
            CT::Title,
            CT::Sentence,
            CT::Alternating,
            CT::Toggle,
            CT::None,
        ]
    }
}

impl CaseType {
    pub fn as_str(&self) -> &'static str {
        use CaseType as CT;
        match self {
            CT::Snake => stringify!(Snake),
            CT::Constant => stringify!(Constant),
            CT::UpperSnake => stringify!(UpperSnake),
            CT::Ada => stringify!(Ada),
            CT::Kebab => stringify!(Kebab),
            CT::Cobol => stringify!(Cobol),
            CT::UpperKebab => stringify!(UpperKebab),
            CT::Train => stringify!(Train),
            CT::Flat => stringify!(Flat),
            CT::UpperFlat => stringify!(UpperFlat),
            CT::Pascal => stringify!(Pascal),
            CT::UpperCamel => stringify!(UpperCamel),
            CT::Camel => stringify!(Camel),
            CT::Lower => stringify!(Lower),
            CT::Upper => stringify!(Upper),
            CT::Title => stringify!(Title),
            CT::Sentence => stringify!(Sentence),
            CT::Alternating => stringify!(Alternating),
            CT::Toggle => stringify!(Toggle),
            CT::None => stringify!(None),
        }
    }
}

impl std::str::FromStr for CaseType {
    type Err = CaseTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = Self::all_cases().into_iter().find(|v| v.as_str() == s);
        if let Some(case_type) = res {
            Ok(case_type)
        } else {
            Err(CaseTypeError::NotACase(s.to_string()))
        }
    }
}

impl ToTokens for CaseType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append(Ident::new(self.as_str(), Span::call_site()));
    }
}

impl AsRef<str> for CaseType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<convert_case::Case<'_>> for CaseType {
    fn from(value: convert_case::Case) -> Self {
        use convert_case::Case as C;
        use CaseType as CT;
        match value {
            C::Snake => CT::Snake,
            C::Constant => CT::Constant,
            C::UpperSnake => CT::UpperSnake,
            C::Ada => CT::Ada,
            C::Kebab => CT::Kebab,
            C::Cobol => CT::Cobol,
            C::UpperKebab => CT::UpperKebab,
            C::Train => CT::Train,
            C::Flat => CT::Flat,
            C::UpperFlat => CT::UpperFlat,
            C::Pascal => CT::Pascal,
            C::UpperCamel => CT::UpperCamel,
            C::Camel => CT::Camel,
            C::Lower => CT::Lower,
            C::Upper => CT::Upper,
            C::Title => CT::Title,
            C::Sentence => CT::Sentence,
            C::Alternating => CT::Alternating,
            C::Toggle => CT::Toggle,
            _ => CT::None,
        }
    }
}

impl From<CaseType> for convert_case::Case<'_> {
    fn from(value: CaseType) -> Self {
        use convert_case::Case as C;
        use CaseType as CT;
        match value {
            CT::Snake => C::Snake,
            CT::Constant => C::Constant,
            CT::UpperSnake => C::UpperSnake,
            CT::Ada => C::Ada,
            CT::Kebab => C::Kebab,
            CT::Cobol => C::Cobol,
            CT::UpperKebab => C::UpperKebab,
            CT::Train => C::Train,
            CT::Flat => C::Flat,
            CT::UpperFlat => C::UpperFlat,
            CT::Pascal => C::Pascal,
            CT::UpperCamel => C::UpperCamel,
            CT::Camel => C::Camel,
            CT::Lower => C::Lower,
            CT::Upper => C::Upper,
            CT::Title => C::Title,
            CT::Sentence => C::Sentence,
            CT::Alternating => C::Alternating,
            CT::Toggle => C::Toggle,
            CT::None => C::Custom {
                boundaries: &[],
                pattern: convert_case::pattern::noop,
                delim: "",
            },
        }
    }
}

impl CaseType {
    pub fn from_attributes(attrs: Vec<Attribute>) -> Option<Self> {
        let mut case_type = None;
        for attr in attrs {
            if let Ok(ct) = CaseType::try_from(attr) {
                case_type = Some(ct);
            }
        }
        case_type
    }
}

impl TryFrom<Attribute> for CaseType {
    type Error = CaseTypeError;

    fn try_from(attr: Attribute) -> Result<Self, Self::Error> {
        match attr.path().get_ident() {
            Some(i) => CaseType::from_str(i.to_string().as_str()),
            None => Err(CaseTypeError::NotACase(format!("{attr:?}"))),
        }
    }
}

#[derive(Debug)]
pub enum CaseTypeError {
    NotACase(String),
    UnknownError,
}

impl std::fmt::Display for CaseTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CaseTypeError as CTE;
        match self {
            CTE::NotACase(case) => write!(f, "Not a valid case: {}", case),
            CTE::UnknownError => write!(f, "Unknown error: Your guess is as good as mine"),
        }
    }
}

impl std::error::Error for CaseTypeError {}
