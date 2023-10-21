pub mod tests;

/// Extracts URLs from a given message containing "twitter.com" or "x.com".
///
/// This function takes a message in the form of a string and searches for URLs within
/// that message. It identifies URLs by checking if they contain either "twitter.com" or
/// "x.com". If such URLs are found, they are collected into a vector and returned.
///
/// # Arguments
///
/// * `message`: A reference to a string containing the message from which to extract URLs.
///
/// # Returns
///
/// A vector of strings containing the identified URLs.
///
/// # Example
///
/// ```
/// use fix_twitterlinks_bot::get_all_urls;
/// let message = "Check out this link: https://twitter.com/example";
/// let urls = get_all_urls(message);
///
/// assert_eq!(urls, vec!["https://twitter.com/example"]);
/// ```
pub fn get_all_urls(message: &str) -> Vec<String> {
    // Create an empty vector to store the identified URLs.
    let mut urls: Vec<String> = vec![];

    // Split the input message into words and check each word for the presence of
    // "twitter.com" or "x.com".
    message.split_whitespace().for_each(|word| {
        if word.contains("twitter.com") || word.contains("x.com") {
            // If a word contains the specified strings, add it to the URLs vector.
            urls.push(word.to_string());
        }
    });

    // Return the vector of identified URLs.
    urls
}

/// Replace specified substrings in a collection of URLs and concatenate them into a single string.
///
/// This function takes a vector of URLs as input, iterates through each URL, and performs
/// substring replacement to modify the URLs. Specifically, it replaces occurrences of
/// "twitter.com" with "fxtwitter.com" and "x.com" with "fixvx.com". The modified URLs are
/// then concatenated into a single string with newline characters ("\n") between them.
///
/// # Arguments
///
/// * `urls`: A vector of strings containing the URLs to be modified.
///
/// # Returns
///
/// A single string that contains the modified URLs concatenated with newline characters.
///
/// # Example
///
/// ```
/// use fix_twitterlinks_bot::replace_all_urls;
/// let urls: Vec<String> = vec!["https://twitter.com/example".into(), "https://x.com/test".into()];
/// let modified_urls = replace_all_urls(urls);
///
/// assert_eq!(modified_urls, "https://fxtwitter.com/example\nhttps://fixvx.com/test\n");
/// ```
pub fn replace_all_urls(urls: Vec<String>) -> String {
    // Clone the input vector to avoid modifying the original URLs.
    urls.clone().iter().fold(String::new(), |acc, url| {
        // Concatenate the accumulator with a newline character and the modified URL.
        acc + {
            // Perform URL modification using pattern matching and the replace method.
            match url.to_string() {
                w if url.contains("twitter.com") => w.replace("twitter.com", "fxtwitter.com") + "\n",
                w if url.contains("x.com") => w.replace("x.com", "fixvx.com") + "\n",
                _ => "".to_string(),
            }
                .as_str()
        }
    })
}