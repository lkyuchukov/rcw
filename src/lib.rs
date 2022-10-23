use std::{error::Error, fs};

use comfy_table::{
    modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS},
    presets::UTF8_FULL,
    Attribute, Cell, CellAlignment, ContentArrangement, Table,
};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let line_count = count_lines(&contents);
    let word_count = count_words(&contents);
    let byte_count = count_bytes(&contents);
    let char_count = count_chars(&contents);

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_SOLID_INNER_BORDERS)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Lines")
                .add_attribute(Attribute::Bold)
                .set_alignment(CellAlignment::Left),
            Cell::new("Words")
                .add_attribute(Attribute::Bold)
                .set_alignment(CellAlignment::Left),
            Cell::new("Bytes")
                .add_attribute(Attribute::Bold)
                .set_alignment(CellAlignment::Left),
            Cell::new("Chars")
                .add_attribute(Attribute::Bold)
                .set_alignment(CellAlignment::Left),
            Cell::new("File")
                .add_attribute(Attribute::Bold)
                .set_alignment(CellAlignment::Left),
        ])
        .add_row(vec![
            Cell::new(line_count.to_string()).set_alignment(CellAlignment::Left),
            Cell::new(word_count.to_string()).set_alignment(CellAlignment::Left),
            Cell::new(byte_count.to_string()).set_alignment(CellAlignment::Left),
            Cell::new(char_count.to_string()).set_alignment(CellAlignment::Left),
            Cell::new(config.file_path).set_alignment(CellAlignment::Left),
        ]);
    println!("{table}");

    Ok(())
}

pub fn count_lines<'a>(contents: &'a str) -> usize {
    contents.lines().count()
}

pub fn count_bytes<'a>(contents: &'a str) -> usize {
    contents.bytes().count()
}

pub fn count_words<'a>(contents: &'a str) -> usize {
    contents.split_whitespace().count()
}

pub fn count_chars<'a>(contents: &'a str) -> usize {
    contents.chars().count()
}

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 1 {
            panic!("not enough arguments");
        }

        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_lines_test() {
        let contents = "\
        The quick brown fox
        jumped over the lazy dog.
        If the dog moved,
        was it really lazy ?";

        assert_eq!(count_lines(contents), 4);
    }

    #[test]
    fn count_bytes_test() {
        let contents = "The quick brown fox jumped over the lazy dog.";
        assert_eq!(count_bytes(contents), 45);
    }

    #[test]
    fn count_words_test() {
        let contents = "The quick brown fox jumped over the lazy dog.";
        assert_eq!(count_words(contents), 9);
    }

    #[test]
    fn count_chars_test() {
        let contents = "The quick brown fox jumped over the lazy dog.";
        assert_eq!(count_words(contents), 9);
    }

}
