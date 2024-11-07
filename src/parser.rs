use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct FunctionDoc {
    pub description: String,
    pub params: Vec<(String, String)>,
    pub return_type: String,
}
pub fn parse_cairo_comments(file_content: &str) -> Vec<FunctionDoc> {
    let mut docs = Vec::new();
    let mut current_doc = FunctionDoc {
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
                // If starting a new function description, save the previous one
                if !current_doc.description.is_empty() {
                    docs.push(current_doc.clone());
                    current_doc = FunctionDoc {
                        description: String::new(),
                        params: Vec::new(),
                        return_type: String::new(),
                    };
                }
                current_doc.description = content.to_string();
                in_params = false;
                in_returns = false;
            } else if in_params {
                // Split parameter line into name and description
                if let Some((name, desc)) = content.split_once(":") {
                    current_doc.params.push((name.trim().to_string(), desc.trim().to_string()));
                }
            } else if in_returns {
                current_doc.return_type = content.to_string();
            } else {
                current_doc.description.push_str(&format!(" {}", content));
            }
        }
    }

    // Push the last parsed function documentation
    if !current_doc.description.is_empty() {
        docs.push(current_doc);
    }

    docs
}
