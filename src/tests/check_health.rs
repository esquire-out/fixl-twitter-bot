use crate::{get_all_urls, replace_all_urls};

/// Asynchronous test to separate a URL from a text message.
///
/// This function takes a text message as input, which may contain a single URL, and extracts and
/// returns that URL.
#[tokio::test]
async fn separate_url_from_text() {
    let message = "Check out this link: https://twitter.com/example";
    let urls = get_all_urls(message);
    assert_eq!(urls, vec!["https://twitter.com/example"]);
}

/// Asynchronous test to separate multiple URLs from a text message.
///
/// This test extracts and returns a vector of URLs found within the input text message.
#[tokio::test]
async fn separate_multiple_urls_from_text() {
    let message = "Check out these links: https://twitter.com/example https://x.com/test they\
    are so cool omg https://invalid-url.com so woah https://something.com//// know what I mean?";
    let urls = get_all_urls(message);
    assert_eq!(
        urls,
        vec!["https://twitter.com/example", "https://x.com/test"]
    );
}

/// Asynchronous test to replace Twitter URLs with modified ones.
///
/// This test takes a vector of URLs and replaces any occurrences of Twitter URLs
/// (e.g., "https://twitter.com/example") by replacing "twitter" with "fxtwitter" in the URLs.
/// The modified URLs are then returned in the vector.
#[tokio::test]
async fn replace_twitter_urls() {
    let urls = vec!["https://twitter.com/example".to_string()];
    let modified_urls = replace_all_urls(urls);
    assert_eq!(modified_urls, "https://fxtwitter.com/example\n");
}

/// Asynchronous test for replacing occurrences of 'x.com' in URLs.
///
/// This test takes a vector of URLs that may contain 'x.com', and it replaces all occurrences
/// of 'x.com' with 'fixvx.com' in each URL. The modified URLs are returned as a vector.
#[tokio::test]
async fn replace_x_urls() {
    let urls = vec!["https://x.com/test".to_string()];
    let modified_urls = replace_all_urls(urls);
    dbg!(&modified_urls);
    assert_eq!(modified_urls, "https://fixvx.com/test\n");
}

/// Asynchronous test test that demonstrates how to ignore invalid URLs.
///
/// This test case uses the `tokio::test` attribute to mark it as an asynchronous test.
/// It constructs a vector of invalid URLs, replaces them, and asserts the result.
#[tokio::test]
async fn ignore_invalid_urls() {
    let urls = vec!["https://invalid-url.com".to_string()];
    let modified_urls = replace_all_urls(urls);
    assert_eq!(modified_urls, "");
}

#[tokio::test]
async fn ignores_fx_urls() {
    let urls = "https://fxtwitter.com/example";
    let invalid_urls = get_all_urls(urls);
    let replaced_urls = replace_all_urls(invalid_urls);
    assert_eq!(replaced_urls, "");
}

#[tokio::test]
async fn ignores_fixvx_urls() {
    let urls = "https://fixvx.com/example";
    let invalid_urls = get_all_urls(urls);
    let replaced_urls = replace_all_urls(invalid_urls);
    assert_eq!(replaced_urls, "");
}

