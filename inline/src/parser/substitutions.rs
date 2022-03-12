

// arrows
const SIMPLE_RIGHT_ARROW: &str = "-->";

pub fn substitute_arrow(possible_arrow: String) -> Option<String> {
  match possible_arrow.as_str() {
    SIMPLE_RIGHT_ARROW => Some("🠖".to_string()),
    _ => None,
  }
}

// emojis
const SMILEY: &str = ":D";

pub fn substitute_emoji(possible_emoji: String) -> Option<String> {
  match possible_emoji.as_str() {
    SMILEY => Some("😃".to_string()),
    _ => None,
  }
}
