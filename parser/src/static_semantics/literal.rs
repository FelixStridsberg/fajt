use crate::error::Result;
use crate::Error;
use fajt_ast::{ArrayElement, LitArray, LitObject, ObjectBinding, PropertyDefinition, Spanned};

impl_trait!(
    impl trait LitArraySemantics for LitArray {
        /// Returns `Err` if not covering `ArrayAssignment`.
        fn assert_covers_assignment_pattern(&self) -> Result<()> {
            let mut elements = self.elements.iter().peekable();

            while let Some(element) = elements.next() {
                if elements.peek().is_none() {
                    break;
                }

                if let ArrayElement::Spread(spread) = element {
                    return Err(Error::syntax_error(
                        "Rest element must be last element".to_owned(),
                        spread.span().clone(),
                    ));
                }
            }

            Ok(())
        }
    }
);

impl_trait!(
    impl trait LitObjectSemantics for LitObject {

        /// Returns `Err` if not covering `ObjectAssignment`.
        fn assert_covers_assignment_pattern(&self) -> Result<()> {
            let mut props = self.props.iter().peekable();
            while let Some(prop) = props.next() {
                if let PropertyDefinition::Method(method) = prop {
                    return Err(Error::syntax_error(
                        "Invalid destructuring assignment target".to_owned(),
                        method.span.clone(),
                    ));
                }

                if props.peek().is_some() {
                    if let PropertyDefinition::Spread(spread) = prop {
                        return Err(Error::syntax_error(
                            "Rest element must be last element".to_owned(),
                            spread.span().clone(),
                        ));
                    }
                }
            }

            Ok(())
        }
    }
);

impl_trait!(
    // The BindingPattern may be used as an object literal as left hand in assignments.
    impl trait LitObjectBindingSemantics for ObjectBinding {
        fn assert_covers_assignment_pattern(&self) -> Result<()> {
            Ok(())
        }
    }
);
