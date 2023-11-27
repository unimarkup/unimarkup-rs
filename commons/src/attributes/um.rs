use crate::lexer::position::Position;

use super::{AttributeValue, NestedAttribute};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UmAttribute {
    Property(UmProperty),
    Nested(Box<NestedAttribute<UmAttribute>>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UmProperty {
    ident: UmPropertyId,
    value: AttributeValue,
    start: Position,
    end: Position,
}

macro_rules! um_attributes {
    ($($name:literal -> $id:ident $(: $info:literal)?);*) => {

        /// Unimarkup attributes that are neither standard CSS or HTML attributes.
        #[derive(Debug, PartialEq, Eq, Clone)]
        pub enum UmPropertyId {
            $(
                #[doc=concat!("The `", $name, "` Unimarkup attribute.")]
                $(
                    #[doc=concat!($info)]
                )?
                $id,

            )*
        }

        impl TryFrom<&str> for UmPropertyId {
            type Error = super::log_id::AttributeError;

            /// Tries to convert a given `str` to an [`UmPropertyId`].
            ///
            /// Usage: `UmPropertyId::try_from("data-lang")`
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                match value.to_lowercase().as_str() {
                    $(
                        $name => Ok(UmPropertyId::$id),
                    )*
                    _ => Err(super::log_id::AttributeError::InvalidHtmlIdent),
                }
            }
        }

        impl UmPropertyId {
            pub fn as_str(&self) -> &str {
                match self {
                    $(
                        UmPropertyId::$id => $name,
                    )*
                }
            }

            #[allow(clippy::len_without_is_empty)]
            pub fn len(&self) -> usize {
                match self {
                    $(
                        UmPropertyId::$id => $name.len(),
                    )*
                }
            }
        }
    }
}

um_attributes!(
    "data-lang" -> DataLang: "Defines the language that is used for highlighting or rendering."
);
