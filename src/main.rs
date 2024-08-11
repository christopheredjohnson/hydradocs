use std::{fs::File, io::BufWriter};
use printpdf::{Mm, PdfDocument, Pt};
use serde::{de::value::Error, Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct PdfDefinition {
    title: String,
    pages: Vec<Page>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Page {
    w: f32,
    h: f32,
}

use serde_json::from_str;

fn parse_json(json_str: &str) -> Result<PdfDefinition, serde_json::Error> {
    from_str(json_str)
}

fn generate_pdf(definition: PdfDefinition, path: String) -> () {
    let mut doc = PdfDocument::empty(definition.title);

    let font = doc.add_builtin_font(printpdf::BuiltinFont::Courier).unwrap();


    for page in definition.pages {
        let w = Pt(page.w);
        let h = Pt(page.h);

        let (page_index, layer_index) = doc.add_page(Mm::from(w), Mm::from(h), "Layer 1");
    }

    doc.save(&mut BufWriter::new(File::create(path).unwrap()))
        .unwrap();
}

fn main() -> Result<(), Error> {
    let json = r#"
    {
        "title": "hydradocs",
        "pages": [
            {
                "w": 792.00,
                "h": 432.00
            },
            {
                "w": 792.00,
                "h": 432.00
            },
            {
                "w": 792.00,
                "h": 432.00
            }
        ]
    }
    "#;

    let res = parse_json(&json);

    match res {
        Ok(definition) => {
            println!("{:?}", definition);

            generate_pdf(definition, "output.pdf".to_string());
        }
        Err(error) => {
            println!("JSON parsing issue: {:?}", error)
        }
    }
    
    Ok(())
}
