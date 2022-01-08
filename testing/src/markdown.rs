use regex::Regex;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

/// Markdown document.
#[derive(Eq, PartialEq, Debug)]
pub struct Markdown<'a> {
    pub sections: Vec<MarkdownSection<'a>>,
}

/// Sections are divided by titles.
#[derive(Eq, PartialEq, Debug)]
pub struct MarkdownSection<'a> {
    pub name: &'a str,
    pub text: Option<&'a str>,
    pub block: Option<MarkdownBlock<'a>>,
}

/// Code block within the markdown document.
#[derive(Eq, PartialEq, Debug)]
pub struct MarkdownBlock<'a> {
    pub language: &'a str,
    pub contents: &'a str,
}

impl MarkdownSection<'_> {
    fn from_string(data: &str) -> Vec<MarkdownSection> {
        let reg = Regex::new(r"(?m)^###").unwrap();
        let sections: Vec<&str> = reg.split(&data).filter(|s| !s.is_empty()).collect();

        sections
            .into_iter()
            .map(|section| {
                let (name, content) = split_title(section);
                let (text, block) = split_code_block(content);

                MarkdownSection {
                    name,
                    text: Some(text),
                    block,
                }
            })
            .collect()
    }
}

fn split_title(str: &str) -> (&str, &str) {
    let (title_line, rest) = str.split_once('\n').unwrap();

    (title_line.trim(), rest.trim())
}

fn split_code_block(str: &str) -> (&str, Option<MarkdownBlock>) {
    let reg = Regex::new(r"(?m)^```").unwrap();
    if let Some(m) = reg.find(str) {
        let start = m.end();
        let length = reg
            .find(&str[start..])
            .expect("Missing end of code block.")
            .start();
        let content = &str[start..(start + length)];

        let (lang, contents) = content.split_once('\n').unwrap();
        let text = &str[..m.start()];
        (
            text.trim(),
            Some(MarkdownBlock {
                language: lang.trim(),
                contents,
            }),
        )
    } else {
        (str, None)
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

impl<'a> Markdown<'a> {
    pub fn from_string(data: &'a str) -> Self {
        let sections = MarkdownSection::from_string(&data);
        Markdown { sections }
    }

    pub fn get_section(&self, name: &str) -> Option<&MarkdownSection> {
        self.sections.iter().find(|s| s.name == name)
    }

    #[allow(unused)]
    pub fn append_json_block(&self, data: &str) {
        //let block = generate_code_block(data, "json");

        //let mut file = OpenOptions::new().write(true).open(&self.path).unwrap();

        //file.seek(SeekFrom::End(0)).unwrap();
        //file.write_all("\n".as_bytes()).unwrap();
        //file.write_all(block.as_bytes()).unwrap();
    }

    #[allow(unused)]
    pub fn replace_json_block(&self, contents: &str) {
        //let data = read_string(&self.path);

        //if let Some((start, end)) = get_code_block_pos(&data, "json") {
        //    let mut new_data = String::new();
        //    new_data.push_str(&data[..start]);
        //    new_data.push_str(contents);
        //    new_data.push_str(&data[end - 1..]);

        //    let mut file = OpenOptions::new().write(true).open(&self.path).unwrap();
        //    file.write_all(new_data.as_bytes()).unwrap();
        //}
    }
}

pub fn read_string(path: &Path) -> String {
    let mut file = File::open(path).expect("Failed to open file.");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Failed to read file.");
    data
}
