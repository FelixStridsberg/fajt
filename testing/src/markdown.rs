mod section;

pub use section::*;

/// Markdown document.
#[derive(Eq, PartialEq, Debug)]
pub struct Markdown<'a> {
    pub sections: Vec<MarkdownSection<'a>>,
}

impl<'a> Markdown<'a> {
    pub fn from_string(data: &'a str) -> Self {
        let sections = MarkdownSection::from_string(&data);
        Markdown { sections }
    }

    pub fn get_section(&self, name: &str) -> Option<&MarkdownSection> {
        self.sections.iter().find(|s| s.name == name)
    }

    pub fn get_block(&self, name: &str) -> Option<&MarkdownBlock> {
        self.get_section(name)
            .map(|section| section.block.as_ref())
            .flatten()
    }

    pub fn get_code(&self, section_name: &str) -> Option<&str> {
        self.get_section(section_name)
            .map(|section| section.block.as_ref().map(|block| block.contents))
            .flatten()
    }

    pub fn set_code(&mut self, section_name: &str, language: &'a str, contents: &'a str) {
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
