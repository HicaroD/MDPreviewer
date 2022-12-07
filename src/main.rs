use actix_web::{get, Responder, HttpResponse, App, HttpServer};
use comrak::{markdown_to_html, ComrakOptions};
use std::{env, fs::File, io::prelude::*, str::FromStr};

fn get_html(markdown_file_content: String) -> String {
    let converted_markdown = markdown_to_html(&markdown_file_content, &ComrakOptions::default());
    converted_markdown
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

fn get_port_number_from_command_line() -> u16 {
    let mut args = env::args().skip(2);

    let port_number = match args.next() {
        // FIXME: avoid too many levels of indentation
        Some(number) => {
            let parsed_number = match u16::from_str(&number) {
                Ok(number) => number,
                Err(_) => {
                    eprintln!("You should a valid port number");
                    std::process::exit(1);
                }
            };
            parsed_number
        },
        None => 8080,
    };

    port_number
}

fn read_markdown_file(file_path: String) -> String {
    // TODO: improve error handling (don't use expect)
    let mut markdown_file = File::open(file_path).expect("Should open file");
    let mut markdown_file_content = String::new();
    markdown_file
        .read_to_string(&mut markdown_file_content)
        .expect("Should read to string");

    markdown_file_content
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
    let port_number = get_port_number_from_command_line();
    println!("You can access this address on your browser: 127.0.0.1:{}", port_number);

    HttpServer::new(|| App::new().service(get_file))
        .bind(("127.0.0.1", port_number))?
        .run()
        .await
}
