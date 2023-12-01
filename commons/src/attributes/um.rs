#[derive(Debug, PartialEq, Clone)]
pub enum UmAttribute<'resolved> {
    /// ```text
    /// {
    ///   single-prop: "data";
    /// }
    /// ```
    Single(UmSingleAttribute<'resolved>),
    /// Allows array-like values for Unimarkup attributes.
    /// Attributes that would be an array of objects in JSON,
    /// have the following syntax:
    ///
    /// ```text
    /// array-prop: {
    ///   field-prop: "some value";
    /// }{
    ///   field-prop: "other value";
    /// }
    /// other-prop: "single value prop";
    /// ```
    ///
    /// Whitespace between `}{` is optional.
    Vec(UmVecAttribute<'resolved>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct UmSingleAttribute<'resolved> {
    ident: UmAttributeId<'resolved>,
    value: &'resolved str,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UmVecAttribute<'resolved> {
    ident: UmAttributeId<'resolved>,
    inner: Vec<UmAttribute<'resolved>>,
}

macro_rules! um_attributes {
    ($($name:literal -> $id:ident $(: $info:literal)?);*) => {

        /// Unimarkup attributes that are neither standard CSS nor HTML attributes.
        #[derive(Debug, PartialEq, Clone)]
        pub enum UmAttributeId<'resolved> {
            $(
                #[doc=concat!("The `", $name, "` Unimarkup attribute.")]
                $(
                    #[doc=concat!($info)]
                )?
                $id,

            )*
            /// Any attribute that is not a Unimarkup attribute.
            ///
            /// **Note:** By default, unknown attributes are considered as CSS.
            /// This is changed in scenarios where only Unimarkup attributes are allowed.
            /// e.g. inside citations `[&&my-id{<only Unimarkup attributes allowed>}]`
            Custom(&'resolved str),
        }

        impl<'resolved> TryFrom<&'resolved str> for UmAttributeId<'resolved> {
            type Error = super::log_id::AttributeError;

            /// Tries to convert a given `str` to an [`UmAttributeId`].
            ///
            /// Usage: `UmAttributeId::try_from("data-lang")`
            fn try_from(value: &'resolved str) -> Result<Self, Self::Error> {
                // TODO: ensure only code points with Unicode ID_Start + ID_Continue property or `-` are part of the string
                // ~regex: "(\p{ID_Start}\p{ID_Continue}*|-(\p{ID_Continue}|-)*\p{ID_Continue})"
                // prevents whitespace and many complex selectors from being taken as Unimarkup attribute property ident.

                match value.to_lowercase().as_str() {
                    $(
                        $name => Ok(UmAttributeId::$id),
                    )*
                    // Parser setting must be available to prevent every attribute from being converted to a Unimarkup attribute.
                    // Default must always be CSS attribute.
                    _ => Ok(UmAttributeId::Custom(value)),
                }
            }
        }

        impl<'resolved> UmAttributeId<'resolved> {
            pub fn as_str(&self) -> &str {
                match self {
                    $(
                        UmAttributeId::$id => $name,
                    )*
                    UmAttributeId::Custom(c) => c,
                }
            }

            #[allow(clippy::len_without_is_empty)]
            pub fn len(&self) -> usize {
                match self {
                    $(
                        UmAttributeId::$id => $name.len(),
                    )*
                    UmAttributeId::Custom(c) => c.len(),
                }
            }
        }
    }
}

um_attributes!(
    "data-lang" -> DataLang: "Defines the language that is used for highlighting or rendering."
);
