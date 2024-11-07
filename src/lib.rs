pub mod html_renderer;
pub mod parser;

use html_renderer::generate_html;
use parser::parse_cairo_comments;

pub fn generate_docs(file_content: &str) -> Result<String, tera::Error> {
    let docs = parse_cairo_comments(file_content);
    generate_html(docs)
}
