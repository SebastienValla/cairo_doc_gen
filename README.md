Here is the complete `README.md` file formatted properly for you to copy and paste:

```markdown
# cairo_doc_gen

A Scarb library for generating HTML documentation from comments in Cairo smart contracts.

## Overview

`cairo_doc_gen` is designed to parse structured comments in Cairo smart contracts and generate an HTML documentation file. This allows developers to document their Cairo contracts directly in the code and easily generate readable documentation for external use.

## Installation

1. Add `cairo_doc_gen` to your Scarb project dependencies:

   ```toml
   [dependencies]
   cairo_doc_gen = { git = "https://github.com/SebastienValla/cairo_doc_gen.git" }
   ```

2. Ensure the `templates/` directory includes `doc_template.html`, the HTML template used for rendering documentation.

## Comment Format

To generate documentation, use structured comments with the following format in your Cairo smart contracts:

- Use `///` for each line of documentation.
- Use section headers `# Parameters` and `# Returns` to organize information.

### Example

Here's an example function with properly structured comments:

```cairo
/// # Transfer Tokens
/// Transfers a specified amount of tokens from the sender to the recipient.
///
/// # Parameters
/// - `sender`: The address of the sender.
/// - `recipient`: The address of the recipient.
/// - `amount`: The number of tokens to transfer.
///
/// # Returns
/// Returns `true` if the transfer was successful, `false` otherwise.
fn transfer(sender: felt252, recipient: felt252, amount: u128) -> bool {
    // function logic here
}
```

### Comment Sections

- **Function Description**: Provide a brief title and description at the beginning of the comment.
- **Parameters Section**: Start with `# Parameters` and list each parameter with its name, followed by a colon and a brief description.
- **Returns Section**: Start with `# Returns`, followed by a description of what the function returns.

## Usage

1. Write structured comments in your Cairo contracts using the format above.

2. Run the documentation generator in your Rust project:

   ```rust
   use cairo_doc_gen::generate_html;
   use cairo_doc_gen::parser::parse_cairo_comments;
   use std::fs;

   fn main() {
       let contract_content = fs::read_to_string("path/to/your_contract.cairo")
           .expect("Unable to read contract file");
       let docs = parse_cairo_comments(&contract_content);
       let html = generate_html(docs).expect("Error generating HTML");
       fs::write("output/contract_documentation.html", html)
           .expect("Unable to write HTML file");
   }
   ```

3. Open `output/contract_documentation.html` to view the generated documentation in a browser.

## Contribution

Feel free to open issues or contribute to this project with suggestions and improvements.


