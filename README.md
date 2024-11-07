# cairo_doc_gen

A Scarb library for generating HTML documentation from comments in Cairo smart contracts.

## Overview

`cairo_doc_gen` is designed to parse structured comments in Cairo smart contracts and generate an HTML documentation file. This allows developers to document their Cairo contracts directly in the code and easily generate readable documentation for external use.

## Installation

You need to have cargo installed

1. Copy Paste `run_docgen.sh` at the root of your project:

2. Ensure your `src/` directory includes all your `.cairo` files

3. ```bash
       chmod -x ./run_docgen.sh
   ```
   your doc will be generated in `./documentation/documentation.html`

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

2. Follow ## Installation instructions

3. Open `output/documentation.html` to view the generated documentation in a browser.

## Contribution

Feel free to open issues or contribute to this project with suggestions and improvements.

## Buy me a coffee 

You like this project ? You can contribute to this project sending found to this  starknet address : 

0x03E1b6776b55A0dcf665a0f538831FA83f7eAB29A3135D16f31DEabd432a1Ec1

ou EVM : 

0x7ea23185a79895fe4717d050ae18aca6da1934dc
