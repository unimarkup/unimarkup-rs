use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum UnimarkupType {
    Heading,
    Paragraph,
    List,
    Verbatim,
    // ... many more to come
}
