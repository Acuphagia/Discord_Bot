use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::async_trait;
use serenity::Client;
use serenity::model::gateway::GatewayIntents;
use std::env;
use tokio::net::TcpListener;
use dotenv::dotenv;
use tokio::task;
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
        println!("Message received: {}", msg
            .content);

        if msg
            .content
                .len() > 200
                    || msg
                        .content
                        .to_lowercase()
                        // Removed the actual bad word for obvious reasons
                        .contains("skibidi")
        {
            println!("Message exceeds 200 characters. Attempting deletion...");

            if let Err(why) = msg
                .delete(&ctx.http)
                .await
            {
                println!("Error deleting message: {:?}", why);
            }
            else
            {
                println!("Message deleted successfully.");
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
        let listener = TcpListener::bind("0.0.0.0:8080")
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
    // Load the environment variables from the .env file
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES 
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
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

