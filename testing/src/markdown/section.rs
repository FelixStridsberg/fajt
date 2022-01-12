use regex::Regex;

#[derive(Eq, PartialEq, Debug)]
pub struct MarkdownSection<'a> {
    pub name: &'a str,
    pub text: Option<&'a str>,
    pub block: Option<MarkdownBlock<'a>>,
}

#[derive(Eq, PartialEq, Debug)]
pub struct MarkdownBlock<'a> {
    pub language: &'a str,
    pub contents: &'a str,
}

impl MarkdownSection<'_> {
    pub fn from_string(data: &str) -> Vec<MarkdownSection> {
        let reg = Regex::new(r"(?m)^###").unwrap();
        reg.split(data)
            .filter(|s| !s.is_empty())
            .map(|section| {
                let (name, content) = split_title(section);
                let (text, block) = split_code_block(content);

                MarkdownSection { name, text, block }
            })
            .collect()
    }

    pub fn get_code(&self) -> Option<&str> {
        self.block.as_ref().map(|block| block.contents)
    }
}

impl ToString for MarkdownBlock<'_> {
    fn to_string(&self) -> String {
        let mut string = String::new();
        string.push_str("```");
        string.push_str(self.language);
        string.push('\n');
        string.push_str(self.contents);

        if self
            .contents
            .chars()
            .last()
            .map(|last| last != '\n')
            .unwrap_or(true)
        {
            string.push('\n');
        }
        string.push_str("```");
        string
    }
}

impl ToString for MarkdownSection<'_> {
    fn to_string(&self) -> String {
        let mut string = String::new();
        string.push_str("### ");
        string.push_str(self.name);
        string.push('\n');

        if let Some(text) = self.text {
            string.push_str(text);
            string.push_str("\n\n");
        }

        if let Some(block) = self.block.as_ref() {
            string.push_str(&block.to_string());
        }

        string
    }
}

fn split_title(str: &str) -> (&str, &str) {
    let (title_line, rest) = str.split_once('\n').unwrap();

    (title_line.trim(), rest.trim())
}

fn split_code_block(str: &str) -> (Option<&str>, Option<MarkdownBlock>) {
    let reg = Regex::new(r"(?m)^```").unwrap();
    if let Some(m) = reg.find(str) {
        let start = m.end();
        let length = reg
            .find(&str[start..])
            .expect("Missing end of code block.")
            .start();
        let content = &str[start..(start + length)];

        let (lang, contents) = content.split_once('\n').unwrap();
        let text = str[..m.start()].trim();
        let text = if text.is_empty() { None } else { Some(text) };

        (
            text,
            Some(MarkdownBlock {
                language: lang.trim(),
                contents,
            }),
        )
    } else {
        let text = if str.is_empty() { None } else { Some(str) };
        (text, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sections() {
        let data = "### Title 1\nsection 2\n```js\nvar a = 1;\n```\n### Title 2\nsection 2\nsection 2 again";
        let sections = MarkdownSection::from_string(data);

        assert_eq!(
            sections,
            vec![
                MarkdownSection {
                    name: "Title 1",
                    text: Some("section 2"),
                    block: Some(MarkdownBlock {
                        language: "js",
                        contents: "var a = 1;\n",
                    })
                },
                MarkdownSection {
                    name: "Title 2",
                    text: Some("section 2\nsection 2 again"),
                    block: None
                }
            ]
        );
    }
}
