

/// Trait for direct substitution
pub trait DirectSubstitution {
  /// Substitutes supported arrows or leaves given input unchanged, if no supported arrow matched.
  /// 
  /// - `possible_arrow` ... String that is tried to be substituted
  fn substitute_arrow(self) -> Self;

  /// Substitutes supported emojis or leaves given input unchanged, if no supported emoji matched.
  /// 
  /// - `possible_emoji` ... String that is tried to be substituted
  fn substitute_emoji(self) -> Self;
}

impl DirectSubstitution for String {
  fn substitute_arrow(self) -> Self {
    match self.as_str() {
      "-->" => "🠖".to_string(),
      "|-->" => "↦".to_string(),
      "---->" => "⟶".to_string(),
      "|---->" => "⟼".to_string(),
      "==>" => "⇒".to_string(),
      "|==>" => "⤇".to_string(),
      "====>" => "⟹".to_string(),
      "|====>" => "⟾".to_string(),
      "<--" => "🠔".to_string(),
      "<--|" => "↤".to_string(),
      "<----" => "⟵".to_string(),
      "<----|" => "⟻".to_string(),
      "<==" => "⇐".to_string(),
      "<==|" => "⤆".to_string(),
      "<====" => "⟸".to_string(),
      "<====|" => "⟽".to_string(),
      "<-->" => "⟷".to_string(),
      "<==>" => "⇔".to_string(),
      _ => self,
    }
  }

  fn substitute_emoji(self) -> Self {
    match self.as_str() {
      ":)" => "🙂".to_string(),
      ";)" => "😉".to_string(),
      ":D" => "😃".to_string(),
      "^^" => "😄".to_string(),
      "=)" => "😊".to_string(),
      ":(" => "🙁".to_string(),
      ";(" => "😢".to_string(),
      ":P" => "😛".to_string(),
      ";P" => "😜".to_string(),
      "O:)" => "😇".to_string(),
      ":O" => "😨".to_string(),
      ">:(" => "🤬".to_string(),
      ":/" => "😕".to_string(),
      "3:)" => "😈".to_string(),
      "--" => "😑".to_string(),
      "<3" => "❤".to_string(),
      "(Y)" => "👍".to_string(),
      "(N)" => "👎".to_string(),
      _ => self,
    }
  }
}
