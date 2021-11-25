use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Serialize, Deserialize, EnumString, Display, PartialEq)]
#[strum(serialize_all = "snake_case")]
pub enum UnimarkupType {
    Heading,
    Paragraph,
    List,
    Verbatim,
    // ... many more to come
}
