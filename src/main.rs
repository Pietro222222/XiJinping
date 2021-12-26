mod config;
use serenity::model::interactions::application_command::ApplicationCommandInteractionDataOptionValue;
use serenity::model::interactions::application_command::ApplicationCommandOptionType;
use serenity::model::prelude::GuildId;
use serenity::model::prelude::Interaction;
use serenity::model::interactions::InteractionResponseType;

use config::bot_config;
use serenity::Client;
use serenity::model::prelude::Ready;
use serenity::client::EventHandler;
use serenity::client::Context;
use serenity::async_trait;
macro_rules! reply {
    ($ctx:ident, $cmd:ident, $content:expr) => {{
         if let Err(why) = $cmd
                .create_interaction_response(&$ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content($content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
    }}
}
struct Handler;
#[async_trait]
impl EventHandler for Handler {
  	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            match command.data.name.as_str() {
                "d" => {
                    let option = command.data.options.get(0).unwrap().resolved.as_ref().unwrap();
                    let _guild_id = command.guild_id.unwrap();
                    let mut member = command.member.as_ref().unwrap().to_owned();

                    match option {
                        ApplicationCommandInteractionDataOptionValue::String(s) => {
                            let cfg = bot_config::Config::new();
                            if *s == cfg.privacy.password {
                                reply!(ctx, command, "Ok.");
                                member.add_role(&ctx, 924717513948954655).await.unwrap()
                            
                            } else {
                                reply!(ctx, command, "https://www.idlememe.com/wp-content/uploads/2021/10/social-credit-meme-idlememe.jpg");
                                return;
                            }
                            tokio::time::sleep(std::time::Duration::from_secs(1 * 60)).await;
                            member.remove_role(&ctx, 924717513948954655).await.unwrap();
                        }
                        _ => unreachable!()
                    }
                }
                unkn => println!("whata fvck: {}", unkn)
            }    
        }
    }
	async fn ready(&self, ctx: Context, ready: Ready) {
		println!("ready mother fucker: {}?", ready.user.name);
		println!("+9999 SOCIAL CREDITS");
		let guild_id = GuildId(924691449562423337);
        let _commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("d").description("+/- SOCIAL CREDITS").create_option(|option| 
                        option.name("password")
                        .description("passwd")
                        .kind(ApplicationCommandOptionType::String)
                        .required(true)
                    )
                })
           });
        _commands.await.unwrap();
	}	
}
#[tokio::main]
async fn main() {
    let cfg = bot_config::Config::new();

     let mut client = Client::builder(cfg.bot.token)
        .event_handler(Handler)
        .application_id(cfg.bot.bot_id)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        eprintln!("Serenity -9999 social credits: {:?}", why);
	}
}
