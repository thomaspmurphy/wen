use regex::Regex; 

fn tokenize(text: &str) -> Vec<String> {
    let re = Regex::new(r"\w+").unwrap();
    re.find_iter(text)
      .map(|m| m.as_str().to_lowercase())
      .collect()
}

fn main() {
    let text = "This is a sample text for tokenisation.";
    let tokens = tokenize(text);
    println!("Tokens: {:?}", tokens);
}
