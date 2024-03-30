use regex::Regex;

/// Tokenizes the input text into a vector of lowercase words.
///
/// # Arguments
///
/// * `text` - A String containing the text to be tokenized.
///
/// # Returns
///
/// A vector of lowercase words extracted from the input text.
///
pub fn tokenise(text: String) -> Vec<String> {
    let re = Regex::new(r"\w+").unwrap();
    re.find_iter(&text)
      .map(|m| m.as_str().to_lowercase())
      .collect()
}
