use crate::config::{DocumentConfig, Element, PageConfig};
use printpdf::*;
use std::{fs::File, io::BufWriter};

pub fn generate_pdf(output_path: String, config: DocumentConfig) {
    let document = PdfDocument::empty(config.title).with_author(config.author);

    for page in config.pages {
        add_page(page, &document);
    }

    document
        .save(&mut BufWriter::new(File::create(output_path).unwrap()))
        .unwrap();
}

pub fn add_page(config: PageConfig, document: &PdfDocumentReference) {
    let (page_index, layer_index) = document.add_page(Mm(100.0), Mm(100.0), "Layer 1");

    let current_layer: PdfLayerReference = document.get_page(page_index).get_layer(layer_index);

    for element in config.elements {
        match element {
            Element::Text {
                content,
                font_size,
                position,
            } => {
                let font = document.add_builtin_font(BuiltinFont::Courier).unwrap();
                current_layer.use_text(content, font_size, Mm(position[0]), Mm(position[1]), &font);
            }
        }
    }
}
