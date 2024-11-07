/// # Test Function
/// This function serves as a test for generating documentation.
///
/// # Parameters
/// - `a`: An integer representing something.
/// - `b`: A boolean indicating a condition.
///
/// # Returns
/// Returns an integer representing the result.
fn test_function(a: felt252, b: bool) -> felt252 {
    return a + (if b { 1 } else { 0 });
}
