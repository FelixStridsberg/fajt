extern crate proc_macro;
extern crate quote;
extern crate syn;

mod enum_from_string;
mod for_each_file;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

use enum_from_string::enum_from_string;

/// Generates FromStr implementation for enum.
/// It can also generate a macro for translating strings to enum variants.
///
/// Example:
/// ```compile_fail
/// #[derive(Eq, Debug, FromString)]
/// #[from_string_macro("animal")]
/// enum Animal {
///     Horse,
///     Cow,
///     #[from_string("piggy")]
///     Pig,
/// }
///
/// // Runtime
/// assert_eq!("horse".parse().unwrap(), Animal::Horse);
/// assert_eq!("piggy".parse().unwrap(), Animal::Pig);
///
/// // Compile time
/// assert_eq!(animal!("horse"), Animal::Horse);
/// assert_eq!(animal!("piggy"), Animal::Horse);
/// ```
///
/// Note: The module using the macro must import the enum, it does not use full path.
///       I.e. it resolves to `Animal::Horse` and not `crate::animal::Animal::Horse`,
///       have not found a way to do that yet. See Custom macro rules for workaround.
///
/// # Custom macro rules
/// from_string_macro_rules allows to add custom rules to the macro.
/// Note: All generated branches always calls it self, so if you add custom rules one of them
///       must accept a variant. Example: `($variant:ident) => { $variant }`
///
/// Example usage:
/// ```compile_fail
/// #[derive(Eq, Debug, FromString)]
/// #[from_string_macro("animal")]
/// #[from_string_macro_rules(
///     ($variant:ident) => {
///         crate::animal::Animal::$variant
///     };
/// )]
/// enum Animal {
///     Horse,
///     Cow,
///     Pig,
/// }
/// ```
#[proc_macro_derive(
    FromString,
    attributes(from_string, from_string_macro, from_string_macro_rules)
)]
pub fn enum_from_string_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let tokens = if let Data::Enum(enum_data) = &input.data {
        enum_from_string(&input, enum_data)
    } else {
        panic!("FromString is only applicable for enums.");
    };

    TokenStream::from(tokens)
}

/// This macro recursively iterates through a directory and calls a macro for each file.
///
/// Example, generating test cases from files:
/// ```compile_fail
/// macro_rules! generate_test_case {
///     ("md", $file_path:literal, $ident:ident) => {
///         #[test]
///         fn $ident() {
///             snapshot_runner($file_path)
///         }
///     };
///     ($extension:literal, $file_path:literal, $ident:ident) => {
///         // Unknown file extensions, ignore...
///     };
/// }
///
/// for_each_file!("parser/tests/snapshots", generate_test_case);
/// ```
#[proc_macro]
pub fn for_each_file(input: TokenStream) -> TokenStream {
    for_each_file::for_each_file(input)
}
