use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::async_trait;
use serenity::Client;
use serenity::model::gateway::GatewayIntents;
use std::env;
use tokio::net::TcpListener;
use dotenv::dotenv;
use once_cell::sync::Lazy;
use std::fs;
use rand::Rng;
use serde::Deserialize;

#[derive(Deserialize)]
struct KindWords(Vec<String>);

static VERY_KIND_WORDS: Lazy<Vec<String>> = Lazy::new(||
    {
    let data = fs
        ::read_to_string("very_kind_words.json")
        .expect("Unable to read file");

    let kind_words: KindWords = serde_json
        ::from_str(&data)
        .expect("JSON was not properly formatted");

    kind_words
        .0
});

static RANDOM_MESSAGES: &[&str] = &[
    "Please keep your messages under 200 characters, & avoid using very bad words.",
    "Your message was too long or contained inappropriate content. Please try again.",
    "Your message was deleted because it was too long or contained inappropriate content.",
    "We appreciate your enthusiasm, but please keep your messages concise and respectful.",
    "Your message was flagged for review due to its length or content. Please adhere to the guidelines.",
    "Your message was removed for being too long or containing inappropriate content. Please be mindful of our community standards.",
    "Your message was deleted because it exceeded the character limit or contained inappropriate content.",
    "Your message was too long or contained inappropriate content. Please try again."
];

fn random_response() -> &'static str
{
    let index = rand
        ::thread_rng()
        .gen_range(0..RANDOM_MESSAGES
            .len());

    RANDOM_MESSAGES[index]
}

struct Handler;

#[async_trait]
impl EventHandler for Handler
{
    async fn message
    (
        &self,
        ctx: Context,
        msg: Message
    )
    {
        println!("Message received: {}", msg.content);

        if msg
            .content
            .len() > 200

            || VERY_KIND_WORDS
                .iter()
                .any(|word|
                    msg
                        .content
                        .to_lowercase()
                        .contains(word))

            || msg
                .content
                .to_lowercase()
                .contains("skibidi")
        {
            println!("Rogue Message Detected. Attempting deletion...");

            if let Err(why) = msg
                .delete(&ctx
                    .http)
                .await
            {
                println!("Error deleting message: {:?}", why);
            }
            else
            {
                println!("Message deleted successfully.");

                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, format!("{}", random_response()))
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
        else
        {
            println!("Message is within acceptable length.");
        }
    }
}

#[tokio::main]
async fn main()
{
    tokio::spawn(async
    {
        let listener = TcpListener
            ::bind("0.0.0.0:8080")
            .await
            .expect("Failed to bind listener");

        println!("Keep-alive server running...");

        loop
        {
            if let Err(e) = listener
                .accept()
                .await
            {
                println!("Failed to accept connection: {:?}", e);
            }
        }
    });

    dotenv()
        .ok();

    let token = env
        ::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let intents = GatewayIntents
        ::GUILD_MESSAGES
        | GatewayIntents
            ::MESSAGE_CONTENT;

    let mut client = Client
        ::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client
        .start()
        .await
    {
        println!("Client error: {:?}", why);
    }
}
