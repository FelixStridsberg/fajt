use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

pub struct TestFile {
    path: PathBuf,
    pub source: String,
    pub ast: Option<String>,
}

impl TestFile {
    pub fn from<P: AsRef<Path>>(path: &P) -> Self {
        let data = read_string(path.as_ref());

        let source = get_code_block(&data, "js")
            .expect("JS input required.")
            .to_owned();

        let ast = get_code_block(&data, "json").map(&str::to_owned);

        TestFile {
            path: PathBuf::from(path.as_ref()),
            source,
            ast,
        }
    }

    pub fn append_json_block(&self, data: &str) {
        let block = generate_code_block(data, "json");

        let mut file = OpenOptions::new().write(true).open(&self.path).unwrap();

        file.seek(SeekFrom::End(0)).unwrap();
        file.write_all("\n".as_bytes()).unwrap();
        file.write_all(block.as_bytes()).unwrap();
    }

    pub fn replace_json_block(&self, contents: &str) {
        let data = read_string(&self.path);

        if let Some((start, end)) = get_code_block_pos(&data, "json") {
            let mut new_data = String::new();
            new_data.push_str(&data[..start]);
            new_data.push_str(contents);
            new_data.push_str(&data[end - 1..]);

            let mut file = OpenOptions::new().write(true).open(&self.path).unwrap();
            file.write_all(new_data.as_bytes()).unwrap();
        }
    }
}

const BLOCK_DELIMITER: &str = "```";

fn get_code_block<'a>(source: &'a str, annotation: &str) -> Option<&'a str> {
    let pos = get_code_block_pos(source, annotation);
    pos.map(|(start, end)| &source[start..end])
}

fn get_code_block_pos(source: &str, annotation: &str) -> Option<(usize, usize)> {
    let block_start = format!("{}{}\n", BLOCK_DELIMITER, annotation);
    if let Some(start) = source.find(&block_start) {
        // Block start without preceding new line is only valid if block starts at first line.
        if start != 0 && &source[start - 1..start] != "\n" {
            return None;
        }

        // The data starts after the start pattern.
        let start = start + block_start.len();
        (&source[start..]).find(BLOCK_DELIMITER).map(|end| {
            // \n``` without a new line after is only valid if file ends
            (start, end + start)
        })
    } else {
        None
    }
}

fn generate_code_block(data: &str, annotation: &str) -> String {
    format!(
        "{}{}\n{}\n{}\n",
        BLOCK_DELIMITER, annotation, data, BLOCK_DELIMITER
    )
}

fn read_string(path: &Path) -> String {
    let mut file = File::open(path).expect("Failed to open file.");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Failed to read file.");
    data
}
