#![doc = include_str!("../README.md")]
extern crate proc_macro;
use convert_case::{self, Casing};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{DeriveInput, Fields, parse_macro_input, spanned::Spanned};

use crate::convert_string::CaseType;

mod convert_string;

/// Implements the [Display](std::fmt::Display) trait for enums. Using [convert_case] crate for the conversion.
///
/// Overall conversion can be applied to the Enum as a whole but can be overiden on specific enum
/// members.
///
/// <div class="warning">The <code>#[Lower]</code> helper attribute adds spaces. Use <code>#[Flat]</code> instead for the equivilent of <code>"MyEnumMember".to_lowercase()</code>.</div>
///
/// | pattern | underscore `_` | hyphen `-` | empty string | space |
/// | ---: | --- | --- | --- | --- |
/// | [lowercase](convert_case::pattern::lowercase) | [snake_case](convert_case::Case::Snake) | [kebab-case](convert_case::Case::Kebab) | [flatcase](convert_case::Case::Flat) | [lower case](convert_case::Case::Lower) |
/// | [uppercase](convert_case::pattern::uppercase) | [CONSTANT_CASE](convert_case::Case::Constant) | [COBOL-CASE](convert_case::Case::Cobol) | [UPPERFLATCASE](convert_case::Case::UpperFlat) | [UPPER CASE](convert_case::Case::Upper) |
/// | [capital](convert_case::pattern::capital) | [Ada_Case](convert_case::Case::Ada) | [Train-Case](convert_case::Case::Train) | [PascalCase](convert_case::Case::Pascal) | [Title Case](convert_case::Case::Title) |
/// | [camel](convert_case::pattern::camel) | | | [camelCase](convert_case::Case::Camel) |
///
/// Look at the documentation for [`Case`](convert_case::Case) for an explanation.
///
/// ```rust
/// # use viking_macros_enum::EnumDisplay;
/// #[derive(EnumDisplay)]
/// #[Flat]
/// enum Orientation {
///    Horizontal,
///    #[UpperFlat]
///    Vertical,
///    #[None]
///    ToTALLyRandDOmCaSe // Needs to remain as is for a completely legit reason ;)
/// }
/// ```
///
/// Is equivalent to:
///
/// ```rust
/// # enum Orientation {
/// #   Horizontal,
/// #   Vertical,
/// #   ToTALLyRandDOmCaSe // Needs to remain as is for a completely legit reason ;)
/// # }
///
/// impl std::fmt::Display for Orientation {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         match self {
///             Orientation::Horizontal => write!(f, "horizontal"),
///             Orientation::Vertical => write!(f, "VERTICAL"),
///             Orientation::ToTALLyRandDOmCaSe => write!(f, "ToTALLyRandDOmCaSe"),
///         }
///     }
/// }
/// ```
///
#[proc_macro_derive(
    EnumDisplay,
    attributes(
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
        None
    )
)]
pub fn derive_enum_display(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let DeriveInput {
        ident, data, attrs, ..
    } = input.clone();
    let transform = CaseType::from_attributes(attrs);
    let enum_data = match data {
        syn::Data::Enum(data_enum) => data_enum,
        _ => {
            return token_stream_error(input.span(), "Must be a enum.");
        }
    };

    let mut enum_items = Vec::new();
    for variant in enum_data.variants {
        let variant_transform = CaseType::from_attributes(variant.attrs);
        let variant_ident = &variant.ident;
        let val = if let Some(ct) = variant_transform {
            variant_ident.to_string().to_case(ct.into())
        } else if let Some(ct) = transform {
            variant_ident.to_string().to_case(ct.into())
        } else {
            variant_ident.to_string()
        };

        let line_quote = quote! {
            #ident::#variant_ident => write!(f, #val)
        };
        enum_items.push(line_quote);
    }

    quote! {
    #[automatically_derived]
    impl ::std::fmt::Display for #ident {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                #(#enum_items),*
            }
        }
    }}
    .into()
}

fn token_stream_error(span: Span, msg: &'static str) -> TokenStream {
    TokenStream::from(syn::Error::new(span, msg).into_compile_error())
}

/// Puts all variants in a [Vec].
///
/// ```no_run
/// #[derive(EnumVec)]
/// enum Test {
///     Compleded,
///     NoTested,
/// }
/// ```
///
/// Will generate:
///
/// ```no_run
/// enum Test {
///     Compleded,
///     NoTested,
/// }
///
/// impl Test {
///    fn all_variants() -> Vec<Self> {
///            vec![Test::Compleded, Test::NoTested];
///    }
/// }
/// ```
///
#[proc_macro_derive(EnumVec)]
pub fn derive_enum_vec(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let DeriveInput { ident, data, .. } = input.clone();

    let enum_data = match data {
        syn::Data::Enum(data_enum) => data_enum,
        _ => {
            return token_stream_error(input.span(), "Must be of type 'enum'.");
        }
    };

    let mut enum_items = Vec::new();
    for variant in enum_data.variants {
        if variant.fields != Fields::Unit {
            return token_stream_error(
                variant.span(),
                "Must be a unit type i.e. no EnumItemTuple | EnumItemStruct.\n enum Foo { Bar, Baz }",
            );
        }
        let variant_ident = &variant.ident;

        enum_items.push(quote! {#ident::#variant_ident});
    }

    let quoted = quote! {
        #[automatically_derived]
        impl #ident {
            fn all_variants() -> Vec<Self> {
                    vec![#(#enum_items),*]
            }
        }
    };
    quoted.into()
}
