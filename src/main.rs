mod config;
mod pdf_generator;
use pdf_generator::generate_pdf;

fn main() {
    let config = config::load_config("test/config.json").expect("Failed to load config");

    generate_pdf("output.pdf".to_string(), config);
}
