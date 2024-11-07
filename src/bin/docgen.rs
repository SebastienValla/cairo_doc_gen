use cairo_doc_gen::html_renderer::generate_html;
use cairo_doc_gen::parser::parse_cairo_comments;
use std::fs::{self, File};
use std::io::Read;

fn main() -> std::io::Result<()> {
    // Vector to hold all the FunctionDocs
    let mut all_docs = Vec::new();

    // Path to the src directory
    let src_path = "./src";

    // Iterate over all .cairo files in src
    for entry in fs::read_dir(src_path)? {
        let entry = entry?;
        let path = entry.path();

        // Only process .cairo files
        if path.extension().and_then(|s| s.to_str()) == Some("cairo") {
            let filename = path.file_name().unwrap().to_string_lossy().to_string();
            let mut file = File::open(&path)?;
            let mut file_content = String::new();
            file.read_to_string(&mut file_content)?;

            // Parse the comments in the file
            let docs = parse_cairo_comments(&file_content, &filename);

            // Collect the docs
            all_docs.extend(docs);
        }
    }

    // Generate the HTML documentation from all docs
    let html = generate_html(all_docs).expect("Error generating HTML");

    // Ensure the output directory exists
    fs::create_dir_all("output")?;

    // Write the HTML to a file
    fs::write("output/documentation.html", html)?;

    println!("Documentation generated at output/documentation.html");

    Ok(())
}
