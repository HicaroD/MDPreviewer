use std::fs::File;
use std::io::prelude::*;
use comrak::{markdown_to_html, ComrakOptions};

fn main() {
    let mut markdown_file_content = String::new();
    let mut markdown_file = File::open("./README.md").expect("Should open file");
    markdown_file.read_to_string(&mut markdown_file_content).expect("Should read to string");

    let converted_markdown = markdown_to_html(&markdown_file_content, &ComrakOptions::default());
    println!("{}", converted_markdown);
}
