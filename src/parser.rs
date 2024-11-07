use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct FunctionDoc {
    pub filename: String,  // Added filename field
    pub description: String,
    pub params: Vec<(String, String)>,
    pub return_type: String,
}
pub fn parse_cairo_comments(file_content: &str, filename: &str) -> Vec<FunctionDoc> {
    let mut docs = Vec::new();
    let mut current_doc = FunctionDoc {
        filename: filename.to_string(),
        description: String::new(),
        params: Vec::new(),
        return_type: String::new(),
    };

    let mut in_params = false;
    let mut in_returns = false;

    for line in file_content.lines() {
        if line.starts_with("///") {
            let content = line[3..].trim();

            if content.starts_with("# Parameters") {
                in_params = true;
                in_returns = false;
            } else if content.starts_with("# Returns") {
                in_params = false;
                in_returns = true;
            } else if content.starts_with("#") {
                // Starting a new function documentation
                if !current_doc.description.is_empty() {
                    docs.push(current_doc.clone());
                    current_doc = FunctionDoc {
                        filename: filename.to_string(),
                        description: String::new(),
                        params: Vec::new(),
                        return_type: String::new(),
                    };
                }
                current_doc.description = content[1..].trim().to_string(); // Remove '#' and trim
                in_params = false;
                in_returns = false;
            } else if in_params {
                if let Some((name, desc)) = content.trim_start_matches('-').trim().split_once(":") {
                    current_doc.params.push((name.trim().to_string(), desc.trim().to_string()));
                }
            } else if in_returns {
                current_doc.return_type = content.to_string();
            } else {
                if !current_doc.description.is_empty() {
                    current_doc.description.push(' ');
                    current_doc.description.push_str(content);
                } else {
                    current_doc.description = content.to_string();
                }
            }
        }
    }

    // Push the last parsed function documentation
    if !current_doc.description.is_empty() {
        docs.push(current_doc);
    }

    docs
}