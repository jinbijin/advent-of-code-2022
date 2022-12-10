use std::str::Chars;

enum TokenCharIterationResult {
    Continue(char),
    Token { from: usize, to: usize },
    EndOfLine,
}

struct TokensState {
    current: usize,
    token_start: usize,
    whitespace_state: Option<usize>,
}

impl TokensState {
    fn process_char(&mut self, c: Option<char>) -> () {
        if let Some(c) = c {
            let char_size = c.len_utf8();
            if self.whitespace_state.is_some() && !c.is_whitespace() {
                self.token_start = self.current;
                self.whitespace_state = None;
            } else if self.whitespace_state.is_none() && c.is_whitespace() {
                self.whitespace_state = Some(self.current);
            }
            self.current += char_size;
        } else {
            self.token_start = self.current;
        }
    }

    fn match_char(&self, c: Option<char>) -> TokenCharIterationResult {
        if let Some(c) = c {
            if let Some(whitespace_state) = self.whitespace_state {
                if !c.is_whitespace() {
                    TokenCharIterationResult::Token {
                        from: self.token_start,
                        to: whitespace_state,
                    }
                } else {
                    TokenCharIterationResult::Continue(c)
                }
            } else {
                TokenCharIterationResult::Continue(c)
            }
        } else {
            let token_end = match self.whitespace_state {
                Some(token_end) => token_end,
                None => self.current,
            };
            if self.token_start >= token_end {
                TokenCharIterationResult::EndOfLine
            } else {
                TokenCharIterationResult::Token {
                    from: self.token_start,
                    to: token_end,
                }
            }
        }
    }
}

pub struct Tokens<'a> {
    source: &'a str,
    chars: Chars<'a>,
    state: TokensState,
}

impl<'a> Tokens<'a> {
    fn next_char(&mut self) -> TokenCharIterationResult {
        let c = self.chars.next();
        let result = self.state.match_char(c);
        self.state.process_char(c);
        result
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = self.next_char();
        while let TokenCharIterationResult::Continue(_) = result {
            result = self.next_char();
        }

        match result {
            TokenCharIterationResult::Token { from, to } => Some(&self.source[from..to]),
            TokenCharIterationResult::EndOfLine => None,
            TokenCharIterationResult::Continue(_) => unreachable!("due to loop end condition"),
        }
    }
}

pub trait AsTokens {
    fn tokens<'a>(&'a self) -> Tokens<'a>;
}

impl AsTokens for &str {
    fn tokens<'a>(&'a self) -> Tokens<'a> {
        Tokens {
            source: self,
            chars: self.chars(),
            state: TokensState {
                current: 0,
                token_start: 0,
                whitespace_state: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn tokens_should_split_single_spaces() -> Result<(), Box<dyn Error>> {
        let tokens = "abc 12 defg".tokens().collect::<Vec<&str>>();
        let expected: Vec<&str> = vec!["abc", "12", "defg"];

        assert_eq!(expected, tokens);
        Ok(())
    }

    #[test]
    fn tokens_should_split_mixed_spaces() -> Result<(), Box<dyn Error>> {
        let tokens = "abc \r12\r defg".tokens().collect::<Vec<&str>>();
        let expected: Vec<&str> = vec!["abc", "12", "defg"];

        assert_eq!(expected, tokens);
        Ok(())
    }

    #[test]
    fn tokens_should_work_with_non_ascii() -> Result<(), Box<dyn Error>> {
        let tokens = "こんにちは 你好".tokens().collect::<Vec<&str>>();
        let expected: Vec<&str> = vec!["こんにちは", "你好"];

        assert_eq!(expected, tokens);
        Ok(())
    }

    #[test]
    fn tokens_of_whitespace_should_be_empty() -> Result<(), Box<dyn Error>> {
        let tokens = "      ".tokens().collect::<Vec<&str>>();
        let expected: Vec<&str> = vec![];

        assert_eq!(expected, tokens);
        Ok(())
    }
}
