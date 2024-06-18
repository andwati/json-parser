# Json parser implimented in Rust

## TODO

### Parsing Basics
 - [ ] objects
 - [ ] arrays
 - [ ] strings
 - [ ] numbers
 - [ ] booleans
 - [ ] null
 - [ ] Handle whitespace and comments
### Support for different data types
 - [ ] Strings (including escaping mechanisms for quotes, backslashes, etc.)
 - [ ] Numbers (integers, floating-point)
 - [ ] Booleans (true, false)
l
 - [ ] Arrays (delimited by square brackets, elements separated by commas)
 - [ ] Objects (delimited by curly braces, key-value pairs with colon separator)

### Error Handling
 - [ ] Report syntax errors (unexpected characters, missing delimiters)
 - [ ] Provide informative error messages for debugging
 - [ ] Optionally consider returning a Result type with specific error structs
### Value Representation
Define Rust data structures to represent parsed JSON values:

 - [ ] Strings
 - [ ] Numbers (consider using appropriate numerical types like i32, f64)
 - [ ] Booleans
 - [ ] Null (e.g., a custom Null type)
 - [ ] Arrays (e.g., Vec<Value>)
 - [ ] Objects (e.g., HashMap<String, Value>)
### Memory Management
 - [ ] Decide on an appropriate memory management strategy (ownership, borrowing)
 - [ ] Consider using String and Vec for owned data
 - [ ] Consider references (&) and slices (&[T]) for borrowed data
### Testing
 - [ ] Write unit tests to verify parsing functionality for different JSON structures
 - [ ] Use a testing framework like cargo test
### (Optional) Advanced Features
 - [ ] Support for comments (if applicable)
 - [ ] Handling large JSON files (consider streaming or incremental parsing)
 - [ ] Dealing with invalid or malformed JSON (custom error handling, recovery mechanisms)
