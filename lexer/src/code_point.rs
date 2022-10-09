use unicode_id::UnicodeID;

/// ECMA Script code points are unicode characters.
pub trait CodePoint {
    fn is_ecma_whitespace(&self) -> bool;
    fn is_ecma_line_terminator(&self) -> bool;
    fn is_start_of_identifier(&self) -> bool;
    fn is_part_of_identifier(&self) -> bool;
}

impl CodePoint for char {
    fn is_ecma_whitespace(&self) -> bool {
        matches!(
            self,
            // Per table in ECMA-262
            '\u{0009}' | '\u{000B}' | '\u{000C}' | '\u{0020}' | '\u{00A0}' | '\u{FEFF}' |
            // Other Zs
            '\u{1680}' | '\u{2000}'..='\u{200A}' | '\u{202F}' | '\u{205F}' | '\u{3000}'
        )
    }

    fn is_ecma_line_terminator(&self) -> bool {
        matches!(self, '\u{000A}' | '\u{000D}' | '\u{2028}' | '\u{2029}')
    }

    fn is_start_of_identifier(&self) -> bool {
        // TODO unicode escape sequence is allowed (ecma-262: 11.8.4)
        matches!(self,  'A'..='Z' | 'a'..='z' | '_' | '$') || self.is_id_start()
    }

    fn is_part_of_identifier(&self) -> bool {
        // TODO unicode escape sequence is allowed (ecma-262: 11.8.4)
        matches!(self, '0'..='9' | 'A'..='Z' | 'a'..='z' | '_' | '$') || self.is_id_continue()
    }
}
