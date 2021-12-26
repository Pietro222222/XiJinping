mod config;
use config::bot_config;
use serenity::Client;
#[tokio::main]
async fn main() {
    let cfg = bot_config::Config::new();

     let mut client = Client::builder(cfg.bot.token)
        //.event_handler(Handler)
        .application_id(cfg.bot.bot_id)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        eprintln!("Serenity -9999 social credits: {:?}", why);
	}
}
