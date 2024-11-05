use std::env;
use dotenv::dotenv;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::builder::CreateMessage;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        // Sets a user to get direct messages as alerts
        // Must be in the format !alertMe
        if msg.content.starts_with("!alertMe ") {

            let response = MessageBuilder::new()
                .push("I will alert ")
                .push_bold_safe(&msg.author.name)
                .push("When it is time for an event.  [TODO]")
                .build();

            if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                println!("Error sending message: {why:?}");
            }
        }

        // Removes a user from getting direct messages as alerts
        // Must be in the format !removeMe
        if msg.content.starts_with("!removeMe ") {
            // TODO Check if they were already getting alerts. If not do nothing.
            let response = MessageBuilder::new()
                .push("I will no longer alert ")
                .push_bold_safe(&msg.author.name)
                .push("When it is time for an event.  [TODO]")
                .build();

            if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                println!("Error sending message: {why:?}");
            }
        }

        // Add new event
        // Must be in the format !newEvent EVENTNAME EVENTDATE(dd/mm/yy or dd/mm)
        if msg.content.starts_with("!newEvent ") {

            let response = MessageBuilder::new()
                .push("User ")
                .push_bold_safe(&msg.author.name)
                .push(" has added the new event: ")
                .push_bold_safe(" TODO ")
                .push(" for ")
                .push_bold_safe(" TODO ")
                .build();

            if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                println!("Error sending message: {why:?}");
            }
        }

        // Ping bot
        if msg.content == "!ping" {
            let channel = match msg.channel_id.to_channel(&context).await {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error getting channel: {why:?}");

                    return;
                },
            };

            let response = MessageBuilder::new()
                .push("User ")
                .mention(&msg.author)
                .push(" used the 'ping' command in the ")
                .mention(&channel)
                .push(" channel")
                .build();

            if let Err(why) = msg.channel_id.say(&context.http, &response).await {
                println!("Error sending message: {why:?}");
            }
        }
        
        // Direct message
        if msg.content == "!messageme" {
            // If the `utils`-feature is enabled, then model structs will have a lot of useful
            // methods implemented, to avoid using an often otherwise bulky Context, or even much
            // lower-level `rest` method.
            //
            // In this case, you can direct message a User directly by simply calling a method on
            // its instance, with the content of the message.
            let builder = CreateMessage::new().content("Hello!");
            let dm = msg.author.dm(&context, builder).await;

            if let Err(why) = dm {
                println!("Error when direct messaging user: {why:?}");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // This line loads the environment variables 

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}