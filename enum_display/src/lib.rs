#![doc = include_str!("../README.md")]
extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{DeriveInput, Fields, parse_macro_input, spanned::Spanned};

macro_rules! modify_string {
    (lowercase, $s:literal) => {
        String::from($s).to_lowercase()
    };

    (uppercase, $s:literal) => {
        String::from($s).to_uppercase()
    };
    (lowercase, $s:ident) => {
        String::from($s).to_lowercase()
    };

    (uppercase, $s:ident) => {
        String::from($s).to_uppercase()
    };

    ($s:literal) => {
        String::from($s)
    };
    ($s:ident) => {
        String::from($s)
    }; // ($i:ident $s:ident) => {{
       //     dbg!($i);
       //     dbg!($s);
       //     compile_error!("Unsure");
       // }};
}

#[derive(Debug, Default, PartialEq)]
enum ModifyStringType {
    #[default]
    None,
    Lowercase,
    Uppercase,
}

/// Implements the 'Display' trait for enums. Enum must only use Unit items.
///
/// Optional helper attribute are available:
/// - lowercase
/// - uppercase
///
/// ```rust
/// use macros::EnumDisplay;
///
/// #[derive(EnumDisplay)]
/// #[lowercase]
/// enum Orientation {
///    Horizontal,
///    Vertical,
/// }
/// ```
/// Is equivalent to:
///
/// ```rust
/// impl std::fmt::Display for Orientation {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         match self {
///             Orientation::Horizontal => write!(f, "horizontal"),
///             Orientation::Vertical => write!(f, "vertical"),
///         }
///     }
/// }
/// ```
///
/// ## Panics
///
/// ```compile_fail
/// use macros::EnumDisplay;
///
/// #[derive(EnumDisplay)]
/// enum Orientation {
///     Horizontal(i64),
///     Vertical { y: i64 },
/// }
/// ```
#[proc_macro_derive(EnumDisplay, attributes(lowercase, uppercase))]
pub fn derive_enum_display(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let DeriveInput {
        ident, data, attrs, ..
    } = input.clone();

    let mut modification: ModifyStringType = ModifyStringType::None;
    for attr in attrs {
        let path_ident = match attr.meta.require_path_only() {
            Ok(p) => p.get_ident(),
            Err(_) => continue,
        };
        if let Some(i) = path_ident {
            match i.to_string().as_str() {
                "lowercase" => {
                    modification = ModifyStringType::Lowercase;
                }
                "uppercase" => {
                    modification = ModifyStringType::Uppercase;
                }
                e => {
                    println!("Didn't add: {e}");
                }
            }
        }
    }
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
        let variant_string = variant_ident.to_string();
        let val = match modification {
            ModifyStringType::None => variant_string,
            ModifyStringType::Lowercase => modify_string!(lowercase, variant_string),
            ModifyStringType::Uppercase => modify_string!(uppercase, variant_string),
        };

        let line_quote = quote! {
            #ident::#variant_ident => write!(f, #val)
        };
        enum_items.push(line_quote);
    }

    quote! {
    impl std::fmt::Display for #ident {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                #(#enum_items),*
            }
        }
    }
            }
    .into()
}

fn token_stream_error(span: Span, msg: &'static str) -> TokenStream {
    TokenStream::from(syn::Error::new(span, msg).into_compile_error())
}
