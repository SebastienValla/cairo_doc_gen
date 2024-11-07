use tera::{Tera, Context};
use crate::parser::FunctionDoc;

pub fn generate_html(docs: Vec<FunctionDoc>) -> Result<String, tera::Error> {
    let template_path = concat!(env!("CARGO_MANIFEST_DIR"), "/templates/*");
    let tera = Tera::new(template_path)?;
    
    let mut context = Context::new();
    context.insert("docs", &docs);

    tera.render("doc_template.html", &context)
}
#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::parser::parse_cairo_comments;
    use std::fs::{self, File};
    use std::io::{self, Read, Write};

    #[test]
    fn test_generate_html_from_contract() -> io::Result<()> {
        // Ensure the output directory exists
        fs::create_dir_all("output")?;

        // Read the contract file content
        let filename = "test_contract.cairo";
        let mut file = File::open(filename)?;
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)?;

        // Extract comments using the parser from your library
        let docs = parse_cairo_comments(&file_content, filename);

        // Generate HTML from the extracted documentation
        let html = generate_html(docs).expect("Error generating HTML");

        // Save the generated HTML to a file for viewing
        let mut output_file = File::create("output/contract_documentation.html")?;
        output_file.write_all(html.as_bytes())?;

        // Check if the HTML contains expected information
        assert!(html.contains("Test Function"));
        assert!(html.contains("An integer representing something."));
        assert!(html.contains("Returns an integer representing the result."));

        Ok(())
    }
}
