use std::env;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;

use fix_twitterlinks_bot::{get_all_urls, replace_all_urls};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    /// Asynchronous message event handler for Discord bot.
    ///
    /// This function is an asynchronous event handler for incoming messages in a Discord channel.
    /// It is responsible for processing messages, validating URLs, and responding to messages that
    /// meet certain criteria.
    ///
    /// # Arguments
    ///
    /// * `self`: A reference to the `Handler` instance.
    /// * `context`: A `Context` instance representing the context of the event.
    /// * `msg`: A `Message` instance representing the incoming message to be processed.
    ///
    /// # Behavior
    ///
    /// The function first checks if the author of the message is a bot. If the author is a bot, the
    /// function returns early without processing the message.
    ///
    /// If the message contains "https://" and also contains either "twitter.com" or "x.com", it
    /// proceeds with the following steps:
    ///
    /// 1. It calls the `get_all_urls` function to extract URLs from the message.
    /// 2. If the `--skip-validation` command line argument is not present, it uses the `reqwest`
    ///    library to validate each URL. Validated URLs are collected, and any validation errors
    ///    are logged to the console.
    /// 3. It then calls the `replace_all_urls` function to replace specific substrings in the URLs,
    ///    such as "twitter.com" with "fxtwitter.com" and "x.com" with "fixvx.com".
    /// 4. It builds a response message using the `MessageBuilder` from the Serenity library and
    ///    sends it to the same channel where the original message was received.
    ///
    /// If there is an error while sending the response message, the error is logged to the console.
    async fn message(&self, context: Context, msg: Message) {
        // Check if the message is sent by a bot; if so, return early without processing.
        if msg.author.bot {
            return;
        } else if msg.content.contains("https://")
            && (msg.content.contains("twitter.com") || msg.content.contains("x.com"))
        {
            // If the message contains "https://" and specific keywords, proceed with processing.

            // Extract URLs from the message using the get_all_urls function.
            let urls = get_all_urls(&msg.content);

            // Check if the "--skip-validation" command line argument is not present.
            let new_urls = if !std::env::args().any(|arg| arg == "--skip-validation") {
                // Create a Reqwest HTTP client for URL validation.
                let client = reqwest::Client::new();

                // Initialize a vector to store validated URLs.
                let mut validated_urls = vec![];

                // Iterate through the URLs and validate each one.
                for url in urls {
                    println!("Validating url: {}", url);

                    // Send an HTTP request to the URL and await the response.
                    let response = client.get(url.as_str()).send().await;

                    match response {
                        Ok(_) => {
                            // If the response is successful, add the URL to the list of validated URLs.
                            validated_urls.push(url.clone());
                            println!("Validated url: {}", url);
                        }
                        Err(why) => {
                            // If there is an error in the HTTP request, log the error.
                            println!("Error getting url: {:?}", why);
                        }
                    }
                }

                // Return the vector of validated URLs.
                validated_urls
            } else {
                // If the "--skip-validation" argument is present, use the original URLs.
                urls
            };

            // Replace specific substrings in the URLs and concatenate them into a single string.
            let fixed_urls = replace_all_urls(new_urls);

            // Build a response message using the Serenity MessageBuilder.
            let response = MessageBuilder::new().push(fixed_urls).build();

            // Send the response message to the same channel where the original message was received.
            if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                // If there is an error sending the message, log the error.
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Modify these Intents as needed.
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
