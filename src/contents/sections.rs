use std::{
    cmp::{self, Ordering},
    str::Chars,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    line_index: usize,
    char_index: usize,
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.char_index.cmp(&other.char_index))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        self.char_index.cmp(&other.char_index)
    }
}

impl Position {
    fn increment(&mut self, c: char) -> () {
        if c == '\n' {
            self.line_index += 1;
        }
        self.char_index += c.len_utf8();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContentSection<'a> {
    pub starts_at_line: usize,
    pub contents: &'a str,
}

#[derive(Debug, Clone, Copy)]
struct EolWhitespaceState {
    end_of_line: Option<Position>,
    carriage_return: Option<Position>,
}

#[derive(Debug, Clone, Copy)]
enum ContentSectionCharIterationResult {
    Continue(char),
    Section { from: Position, to: Position },
    EndOfContent,
}

#[derive(Debug, Clone, Copy)]
struct SectionsState {
    current: Position,
    section_start: Position,
    eol_whitespace_state: EolWhitespaceState,
}

impl SectionsState {
    fn process_char(&mut self, c: Option<char>) -> () {
        if let Some(c) = c {
            if c == '\n' {
                if let Some(_) = self.eol_whitespace_state.end_of_line {
                    self.section_start = Position {
                        line_index: self.current.line_index + 1,
                        char_index: self.current.char_index + 1,
                    };
                }
                self.eol_whitespace_state.end_of_line = Some(
                    self.eol_whitespace_state
                        .carriage_return
                        .map_or(self.current, |x| x),
                );
                self.eol_whitespace_state.carriage_return = None;
            } else if c == '\r' {
                self.eol_whitespace_state.carriage_return = Some(self.current);
            } else if c.is_whitespace() {
                self.eol_whitespace_state.carriage_return = None;
            } else {
                self.eol_whitespace_state.carriage_return = None;
                self.eol_whitespace_state.end_of_line = None;
            }
            self.current.increment(c);
        } else {
            self.section_start = self.current;
        }
    }

    fn match_char(&self, c: Option<char>) -> ContentSectionCharIterationResult {
        if let Some(c) = c {
            if let Some(end_of_line) = self.eol_whitespace_state.end_of_line {
                if c == '\n' {
                    self.get_section_double_eol(end_of_line)
                } else {
                    ContentSectionCharIterationResult::Continue(c)
                }
            } else {
                ContentSectionCharIterationResult::Continue(c)
            }
        } else {
            let section_end = match self.eol_whitespace_state.end_of_line {
                Some(section_end) => section_end,
                None => self.current,
            };
            if self.section_start >= section_end {
                ContentSectionCharIterationResult::EndOfContent
            } else {
                ContentSectionCharIterationResult::Section {
                    from: self.section_start,
                    to: section_end,
                }
            }
        }
    }

    fn get_section_double_eol(&self, section_end: Position) -> ContentSectionCharIterationResult {
        ContentSectionCharIterationResult::Section {
            from: self.section_start,
            to: cmp::max(self.section_start, section_end),
        }
    }
}

#[derive(Debug, Clone)]
struct Sections<'a> {
    source: &'a str,
    chars: Chars<'a>,
    state: SectionsState,
}

impl<'a> Sections<'a> {
    fn next_char(&mut self) -> ContentSectionCharIterationResult {
        let c = self.chars.next();
        let result = self.state.match_char(c);
        self.state.process_char(c);
        result
    }
}

impl<'a> Iterator for Sections<'a> {
    type Item = ContentSection<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = self.next_char();
        while let ContentSectionCharIterationResult::Continue(_) = result {
            result = self.next_char();
        }

        match result {
            ContentSectionCharIterationResult::EndOfContent => None,
            ContentSectionCharIterationResult::Section { from, to } => Some(ContentSection {
                starts_at_line: from.line_index,
                contents: &self.source[from.char_index..to.char_index],
            }),
            ContentSectionCharIterationResult::Continue(_) => {
                unreachable!("Loop ran until result is not in this case.")
            }
        }
    }
}

trait AsSections<'a> {
    fn sections(&self) -> Sections<'a>;
}

impl<'a> AsSections<'a> for &'a str {
    fn sections(&self) -> Sections<'a> {
        Sections {
            source: self,
            chars: self.chars(),
            state: SectionsState {
                current: Position {
                    line_index: 0,
                    char_index: 0,
                },
                section_start: Position {
                    line_index: 0,
                    char_index: 0,
                },
                eol_whitespace_state: EolWhitespaceState {
                    end_of_line: None,
                    carriage_return: None,
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn sections_should_split_lf() -> Result<(), Box<dyn Error>> {
        let sections = "a\nb\nc\n\nd\n\ne\nf\n\n\ng\n\n"
            .sections()
            .collect::<Vec<ContentSection>>();
        let expected: Vec<ContentSection> = vec![
            ContentSection {
                starts_at_line: 0,
                contents: "a\nb\nc",
            },
            ContentSection {
                starts_at_line: 4,
                contents: "d",
            },
            ContentSection {
                starts_at_line: 6,
                contents: "e\nf",
            },
            ContentSection {
                starts_at_line: 9,
                contents: "",
            },
            ContentSection {
                starts_at_line: 10,
                contents: "g",
            },
        ];

        assert_eq!(expected, sections);
        Ok(())
    }

    #[test]
    fn sections_should_split_crlf() -> Result<(), Box<dyn Error>> {
        let sections = "a\r\nb\r\nc\r\n\r\nd\r\n\r\ne\r\nf\r\n\r\n\r\ng\r\n\r\n"
            .sections()
            .collect::<Vec<ContentSection>>();
        let expected: Vec<ContentSection> = vec![
            ContentSection {
                starts_at_line: 0,
                contents: "a\r\nb\r\nc",
            },
            ContentSection {
                starts_at_line: 4,
                contents: "d",
            },
            ContentSection {
                starts_at_line: 6,
                contents: "e\r\nf",
            },
            ContentSection {
                starts_at_line: 9,
                contents: "",
            },
            ContentSection {
                starts_at_line: 10,
                contents: "g",
            },
        ];

        assert_eq!(expected, sections);
        Ok(())
    }

    #[test]
    fn sections_should_leave_extra_trailing_crs() -> Result<(), Box<dyn Error>> {
        let sections = "abc\r\r\n\ndef".sections().collect::<Vec<ContentSection>>();
        let expected: Vec<ContentSection> = vec![
            ContentSection {
                starts_at_line: 0,
                contents: "abc\r",
            },
            ContentSection {
                starts_at_line: 2,
                contents: "def",
            },
        ];

        assert_eq!(expected, sections);
        Ok(())
    }

    #[test]
    fn sections_of_empty_should_be_empty() -> Result<(), Box<dyn Error>> {
        let sections = "".sections().collect::<Vec<ContentSection>>();
        let expected: Vec<ContentSection> = vec![];

        assert_eq!(expected, sections);
        Ok(())
    }

    #[test]
    fn sections_of_white_line_should_be_empty() -> Result<(), Box<dyn Error>> {
        let sections = "\n".sections().collect::<Vec<ContentSection>>();
        let expected: Vec<ContentSection> = vec![];

        assert_eq!(expected, sections);
        Ok(())
    }

    #[test]
    fn sections_of_double_white_line_should_have_one_section() -> Result<(), Box<dyn Error>> {
        let sections = "\n\n".sections().collect::<Vec<ContentSection>>();
        let expected: Vec<ContentSection> = vec![ContentSection {
            starts_at_line: 0,
            contents: "",
        }];

        assert_eq!(expected, sections);
        Ok(())
    }

    #[test]
    fn sections_of_contiguous_lines_should_have_one_section() -> Result<(), Box<dyn Error>> {
        let sections = "a\nb\nc\nd".sections().collect::<Vec<ContentSection>>();
        let expected: Vec<ContentSection> = vec![ContentSection {
            starts_at_line: 0,
            contents: "a\nb\nc\nd",
        }];

        assert_eq!(expected, sections);
        Ok(())
    }

    #[test]
    fn sections_of_terminated_contiguous_lines_should_have_one_section(
    ) -> Result<(), Box<dyn Error>> {
        let sections = "a\nb\nc\nd\n".sections().collect::<Vec<ContentSection>>();
        let expected: Vec<ContentSection> = vec![ContentSection {
            starts_at_line: 0,
            contents: "a\nb\nc\nd",
        }];

        assert_eq!(expected, sections);
        Ok(())
    }

    #[test]
    fn sections_of_doubly_terminated_contiguous_lines_should_have_one_section(
    ) -> Result<(), Box<dyn Error>> {
        let sections = "a\nb\nc\nd\n\n".sections().collect::<Vec<ContentSection>>();
        let expected: Vec<ContentSection> = vec![ContentSection {
            starts_at_line: 0,
            contents: "a\nb\nc\nd",
        }];

        assert_eq!(expected, sections);
        Ok(())
    }

    #[test]
    fn sections_of_triply_terminated_contiguous_lines_should_have_two_sections(
    ) -> Result<(), Box<dyn Error>> {
        let sections = "a\nb\nc\nd\n\n\n"
            .sections()
            .collect::<Vec<ContentSection>>();
        let expected: Vec<ContentSection> = vec![
            ContentSection {
                starts_at_line: 0,
                contents: "a\nb\nc\nd",
            },
            ContentSection {
                starts_at_line: 5,
                contents: "",
            },
        ];

        assert_eq!(expected, sections);
        Ok(())
    }
}
