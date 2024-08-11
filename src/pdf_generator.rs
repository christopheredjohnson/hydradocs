use std::{fs::File, io::BufWriter};
use printpdf::*;
use crate::config::DocumentConfig;


pub fn generate_pdf (output_path: String, config: DocumentConfig) {

    println!("{:?}", config);

    let document = PdfDocument::empty(config.title).with_author(config.author);

    let (page_index, layer_index) =  document.add_page(Mm(100.0), Mm(100.0), "Layer 1");

    document.save(&mut BufWriter::new(File::create(output_path).unwrap())).unwrap();
}