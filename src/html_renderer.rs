use crate::parser::FunctionDoc;
use tera::{Context, Tera};

pub fn generate_html(docs: Vec<FunctionDoc>) -> Result<String, tera::Error> {
    let tera = Tera::new("templates/*")?; // Remove `mut` here

    let mut context = Context::new(); // Add `mut` here
    context.insert("docs", &docs);

    tera.render("doc_template.html", &context)
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::parser::parse_cairo_comments;
    use std::fs::File;
    use std::io::{self, Read, Write};

    #[test]
    fn test_generate_html_from_contract() -> io::Result<()> {
        // Read the contract file content
        let mut file = File::open("./test_contract.cairo")?;
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)?;

        // Extract comments using the parser from your library
        let docs = parse_cairo_comments(&file_content);

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
