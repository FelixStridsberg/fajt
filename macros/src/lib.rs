extern crate proc_macro;
extern crate quote;
extern crate syn;

mod enum_from_string;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

use enum_from_string::enum_from_string;

/// Generates FromStr implementation for enum.
/// It can also generate a macro for translating strings to enum variants.
///
/// Example:
/// ```no_run
/// #[derive(FromString)]
/// #[from_string_macro("animal")]
/// enum Animal {
///     Horse,
///     Cow,
///     #[from_string("piggy")]
///     Pig,
/// }
///
/// // Runtime
/// assert_eq!("horse".parse(), Animal::Horse);
/// assert_eq!("piggy".parse(), Animal::Pig);
///
/// // Compile time
/// assert_eq!(animal!("horse"), Animal::Horse);
/// assert_eq!(animal!("piggy"), Animal::Horse);
/// ```
///
/// Note: The module using the macro must import the enum, it does not use full path.
///       I.e. it resolves to `Animal::Horse` and not `crate::animal::Animal::Horse`,
///       have not found a way to do that yet.
///
/// # Wrap macro
/// from_string_macro_wrap allows to call a custom macro around each variant. When used
/// the generated macro will look like this:
/// ```no_run
///
/// // Define the wrap_animal! macro here.
///
/// #[derive(FromString)]
/// #[from_string_macro("animal")]
/// #[from_string_macro_wrap(wrap_animal)]
/// enum Animal {
///     Horse,
///     Cow,
///     Pig,
/// }
///
/// // Output will be equivalent to the following:
/// macro_rules! animal {
///     ("horse") => { wrap_animal!(Animal::Horse) };
///     ("cow") => { wrap_animal!(Animal::Cow) };
///     ...
/// }
/// ```
/// This allows to customize the data that is returned by the macro.
#[proc_macro_derive(FromString, attributes(from_string, from_string_macro, from_string_macro_wrap))]
pub fn enum_from_string_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let tokens = if let Data::Enum(enum_data) = &input.data {
        enum_from_string(&input, &enum_data)
    } else {
        panic!("FromString is only applicable for enums.");
    };

    TokenStream::from(tokens)
}
