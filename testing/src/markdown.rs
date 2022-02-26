mod section;

pub use section::*;

/// Representation of a simple markdown document.
/// The document contains sections divided by titles (h3, aka ###). Each section can contain some
/// optional text and an optional code block.
#[derive(Eq, PartialEq, Debug)]
pub struct Markdown<'a> {
    pub sections: Vec<MarkdownSection<'a>>,
}

impl<'a> Markdown<'a> {
    pub fn from_string(data: &'a str) -> Self {
        let sections = MarkdownSection::from_string(data);
        Markdown { sections }
    }

    pub fn get_section(&self, name: &str) -> Option<&MarkdownSection> {
        self.sections.iter().find(|s| s.name == name)
    }

    pub fn get_block(&self, section_name: &str) -> Option<&MarkdownBlock> {
        self.get_section(section_name)
            .and_then(|section| section.block.as_ref())
    }

    pub fn get_block_content(&self, section_name: &str) -> Option<&str> {
        self.get_block(section_name).map(|block| block.contents)
    }

    pub fn set_block_content(&mut self, section_name: &str, language: &'a str, contents: &'a str) {
        if let Some(section) = self.sections.iter_mut().find(|s| s.name == section_name) {
            section.block = Some(MarkdownBlock { language, contents });
        }
    }
}

impl ToString for Markdown<'_> {
    fn to_string(&self) -> String {
        let sections: Vec<String> = self.sections.iter().map(|s| s.to_string()).collect();
        sections.join("\n\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let document = "\
            ### Only text section\n\
            Optional text\n\
            \n\
            ### Only code block section\n\
            ```js\n\
            var a = 1;\n\
            ```\n\
            ### Empty section
        ";

        let markdown = Markdown::from_string(document);

        assert_eq!(
            markdown,
            Markdown {
                sections: vec![
                    MarkdownSection {
                        name: "Only text section",
                        text: Some("Optional text"),
                        block: None
                    },
                    MarkdownSection {
                        name: "Only code block section",
                        text: None,
                        block: Some(MarkdownBlock {
                            language: "js",
                            contents: "var a = 1;\n"
                        })
                    },
                    MarkdownSection {
                        name: "Empty section",
                        text: None,
                        block: None
                    }
                ]
            }
        );
    }

    #[test]
    fn to_string() {
        let markdown = Markdown {
            sections: vec![
                MarkdownSection {
                    name: "Only text section",
                    text: Some("Optional text"),
                    block: None,
                },
                MarkdownSection {
                    name: "Only code block section",
                    text: None,
                    block: Some(MarkdownBlock {
                        language: "js",
                        contents: "var a = 1;\n",
                    }),
                },
                MarkdownSection {
                    name: "Empty section",
                    text: None,
                    block: None,
                },
            ],
        };

        let expected_string = "\
            ### Only text section\n\
            Optional text\n\
            \n\n\n\
            ### Only code block section\n\
            ```js\n\
            var a = 1;\n\
            ```\n\
            \n\
            ### Empty section\n\
        ";

        assert_eq!(markdown.to_string(), expected_string);
    }
}
