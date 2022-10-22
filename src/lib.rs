use std::{error::Error, fs};

use comfy_table::{
    modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS},
    presets::UTF8_FULL,
    Attribute, Cell, CellAlignment, ContentArrangement, Table, Color,
};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let line_count = count_lines(&contents);
    let word_count = count_words(&contents);
    let byte_count = count_bytes(&contents);

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_SOLID_INNER_BORDERS)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Lines").add_attribute(Attribute::Bold).fg(Color::DarkCyan),
            Cell::new("Words").add_attribute(Attribute::Bold).fg(Color::DarkCyan),
            Cell::new("Bytes").add_attribute(Attribute::Bold).fg(Color::DarkCyan),
            Cell::new("Filename").add_attribute(Attribute::Bold).fg(Color::DarkCyan),
        ])
        .add_row(vec![
            Cell::new(line_count.to_string()).set_alignment(CellAlignment::Right),
            Cell::new(word_count.to_string()).set_alignment(CellAlignment::Right),
            Cell::new(byte_count.to_string()).set_alignment(CellAlignment::Right),
            Cell::new(config.file_path).set_alignment(CellAlignment::Right),
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
    let mut word_count = 0;
    let contents = contents.trim_end();
    for ch in contents.chars() {
        if ch.is_whitespace() {
            word_count += 1;
        }
    }

    return word_count;
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
