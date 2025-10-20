use std::collections::HashMap;

/// C64 BASIC V2 tokens (0x80-0xCB)
pub struct TokenTable {
    token_to_keyword: HashMap<u8, &'static str>,
    keyword_to_token: HashMap<&'static str, u8>,
}

impl TokenTable {
    pub fn new() -> Self {
        let mut token_to_keyword = HashMap::new();
        let mut keyword_to_token = HashMap::new();

        // C64 BASIC V2 Keywords (0x80-0xA2: Commands)
        let commands = vec![
            (0x80, "END"),
            (0x81, "FOR"),
            (0x82, "NEXT"),
            (0x83, "DATA"),
            (0x84, "INPUT#"),
            (0x85, "INPUT"),
            (0x86, "DIM"),
            (0x87, "READ"),
            (0x88, "LET"),
            (0x89, "GOTO"),
            (0x8A, "RUN"),
            (0x8B, "IF"),
            (0x8C, "RESTORE"),
            (0x8D, "GOSUB"),
            (0x8E, "RETURN"),
            (0x8F, "REM"),
            (0x90, "STOP"),
            (0x91, "ON"),
            (0x92, "WAIT"),
            (0x93, "LOAD"),
            (0x94, "SAVE"),
            (0x95, "VERIFY"),
            (0x96, "DEF"),
            (0x97, "POKE"),
            (0x98, "PRINT#"),
            (0x99, "PRINT"),
            (0x9A, "CONT"),
            (0x9B, "LIST"),
            (0x9C, "CLR"),
            (0x9D, "CMD"),
            (0x9E, "SYS"),
            (0x9F, "OPEN"),
            (0xA0, "CLOSE"),
            (0xA1, "GET"),
            (0xA2, "NEW"),
        ];

        // Operators (0xAA-0xB3)
        let operators = vec![
            (0xAA, "TAB("),
            (0xAB, "TO"),
            (0xAC, "FN"),
            (0xAD, "SPC("),
            (0xAE, "THEN"),
            (0xAF, "NOT"),
            (0xB0, "STEP"),
            (0xB1, "+"),
            (0xB2, "-"),
            (0xB3, "*"),
        ];

        // Functions (0xB4-0xCB)
        let functions = vec![
            (0xB4, "/"),
            (0xB5, "^"),
            (0xB6, "AND"),
            (0xB7, "OR"),
            (0xB8, ">"),
            (0xB9, "="),
            (0xBA, "<"),
            (0xBB, "SGN"),
            (0xBC, "INT"),
            (0xBD, "ABS"),
            (0xBE, "USR"),
            (0xBF, "FRE"),
            (0xC0, "POS"),
            (0xC1, "SQR"),
            (0xC2, "RND"),
            (0xC3, "LOG"),
            (0xC4, "EXP"),
            (0xC5, "COS"),
            (0xC6, "SIN"),
            (0xC7, "TAN"),
            (0xC8, "ATN"),
            (0xC9, "PEEK"),
            (0xCA, "LEN"),
            (0xCB, "STR$"),
            (0xCC, "VAL"),
            (0xCD, "ASC"),
            (0xCE, "CHR$"),
            (0xCF, "LEFT$"),
            (0xD0, "RIGHT$"),
            (0xD1, "MID$"),
        ];

        // Populate both maps
        for (token, keyword) in commands.iter().chain(operators.iter()).chain(functions.iter()) {
            token_to_keyword.insert(*token, *keyword);
            keyword_to_token.insert(*keyword, *token);
        }

        TokenTable {
            token_to_keyword,
            keyword_to_token,
        }
    }

    pub fn get_keyword(&self, token: u8) -> Option<&'static str> {
        self.token_to_keyword.get(&token).copied()
    }

    #[allow(dead_code)]
    pub fn get_token(&self, keyword: &str) -> Option<u8> {
        self.keyword_to_token.get(keyword).copied()
    }

    /// Detokenize a line of tokenized BASIC
    pub fn detokenize(&self, tokenized: &[u8]) -> String {
        let mut result = String::new();
        let mut i = 0;

        while i < tokenized.len() {
            let byte = tokenized[i];

            if byte == 0 {
                break; // End of line
            } else if byte >= 0x80 {
                // Token
                if let Some(keyword) = self.get_keyword(byte) {
                    result.push_str(keyword);
                    result.push(' ');
                }
            } else if byte == 0x22 {
                // Quote - copy until next quote
                result.push('"');
                i += 1;
                while i < tokenized.len() && tokenized[i] != 0x22 && tokenized[i] != 0 {
                    result.push(tokenized[i] as char);
                    i += 1;
                }
                if i < tokenized.len() && tokenized[i] == 0x22 {
                    result.push('"');
                }
            } else {
                // Regular character (PETSCII)
                result.push(byte as char);
            }
            i += 1;
        }

        result.trim_end().to_string()
    }

    /// Tokenize a line of BASIC text
    pub fn tokenize(&self, line: &str) -> Vec<u8> {
        let mut result = Vec::new();
        let upper = line.to_uppercase();
        let mut i = 0;
        let bytes = upper.as_bytes();

        while i < bytes.len() {
            // Check if we're in a string literal
            if bytes[i] == b'"' {
                result.push(b'"');
                i += 1;
                while i < bytes.len() && bytes[i] != b'"' {
                    result.push(bytes[i]);
                    i += 1;
                }
                if i < bytes.len() {
                    result.push(b'"');
                    i += 1;
                }
                continue;
            }

            // Try to match a keyword
            let mut matched = false;
            for keyword in self.keyword_to_token.keys() {
                if upper[i..].starts_with(keyword) {
                    // Check if it's a complete word (not part of a variable name)
                    let after_idx = i + keyword.len();
                    let is_complete = after_idx >= bytes.len()
                        || !bytes[after_idx].is_ascii_alphanumeric();

                    if is_complete {
                        result.push(self.keyword_to_token[keyword]);
                        i += keyword.len();
                        matched = true;
                        break;
                    }
                }
            }

            if !matched {
                result.push(bytes[i]);
                i += 1;
            }
        }

        result.push(0); // End of line marker
        result
    }
}
