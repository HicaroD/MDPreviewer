use actix_web::{get, Responder, HttpResponse, App, HttpServer};
use comrak::{markdown_to_html, ComrakOptions};
use std::{env, fs::File, io::prelude::*};

fn get_html(markdown_file_content: String) -> String {
    let converted_markdown = markdown_to_html(&markdown_file_content, &ComrakOptions::default());
    return converted_markdown;
}

fn get_file_path_from_command_line() -> String {
    let mut args = env::args().skip(1);

    let path = match args.next() {
        Some(path) => path,
        None => {
            eprintln!("You should set a path for a Markdown file in the command line argument");
            std::process::exit(1);
        }
    };
    return path;
}

fn read_markdown_file(file_path: String) -> String {
    // TODO: improve error handling (don't use expect)
    let mut markdown_file = File::open(file_path).expect("Should open file");
    let mut markdown_file_content = String::new();
    markdown_file
        .read_to_string(&mut markdown_file_content)
        .expect("Should read to string");

    return markdown_file_content;
}

#[get("/")]
async fn get_file() -> impl Responder {
    let file_path = get_file_path_from_command_line();
    let markdown_file_content = read_markdown_file(file_path);
    let html = get_html(markdown_file_content);

    HttpResponse::Ok().body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("You can access this address on your browser: 127.0.0.1:5110");
    HttpServer::new(|| App::new().service(get_file))
        .bind(("127.0.0.1", 5110))?
        .run()
        .await
}
