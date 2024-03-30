extern crate wen;
use wen::tokeniser::tokenise as t;

#[test]
fn test_tokenizer() {
    // Test input
    let text = "This is a test string for tokenization.".to_string();

    // Expected output
    let expected_tokens = vec![
        "this".to_string(),
        "is".to_string(),
        "a".to_string(),
        "test".to_string(),
        "string".to_string(),
        "for".to_string(),
        "tokenization".to_string(),
    ];

    // Call the tokenize function
    let tokens = t(text.clone());

    // Assert that the actual tokens match the expected tokens
    assert_eq!(tokens, expected_tokens);
}
