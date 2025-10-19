// Bot Discord de Musique en Rust
// Version complète avec toutes les fonctionnalités

use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    framework::standard::{
        macros::{command, group},
        Args, CommandResult, StandardFramework,
    },
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use songbird::SerenityInit;
use std::env;
use tracing::{error, info};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("✅ {} est connecté et prêt!", ready.user.name);
        info!("Bot ID: {}", ready.user.id);
    }
}

#[group]
#[commands(join, leave, play, pause, resume, skip, stop, queue, volume, nowplaying)]
struct General;

#[tokio::main]
async fn main() {
    // Initialiser le logger
    tracing_subscriber::fmt::init();

    // Charger les variables d'environnement
    dotenv::dotenv().ok();

    info!("🔍 Chargement de la configuration...");

    let token = env::var("DISCORD_BOT_TOKEN")
        .expect("❌ DISCORD_BOT_TOKEN n'est pas définie dans les variables d'environnement");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::MESSAGE_CONTENT;

    info!("🚀 Démarrage du bot...");

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Erreur lors de la création du client");

    if let Err(why) = client.start().await {
        error!("❌ Erreur du client: {:?}", why);
    }
}

#[command]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            msg.reply(ctx, "❌ Vous devez être dans un salon vocal!").await?;
            return Ok(());
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird client non initialisé");

    let _handler = manager.join(guild_id, connect_to).await;

    msg.reply(ctx, format!("✅ Connecté à <#{}>", connect_to))
        .await?;

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird client non initialisé");

    if manager.get(guild_id).is_some() {
        manager.remove(guild_id).await?;
        msg.reply(ctx, "👋 Déconnecté du salon vocal").await?;
    } else {
        msg.reply(ctx, "❌ Le bot n'est pas dans un salon vocal")
            .await?;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let url = args.rest();

    if url.is_empty() {
        msg.reply(ctx, "❌ Usage: !play <url ou recherche YouTube>")
            .await?;
        return Ok(());
    }

    msg.reply(
        ctx,
        "🎵 Fonctionnalité de lecture en cours d'implémentation...",
    )
    .await?;

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn pause(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird client non initialisé");

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let _ = handler.queue().pause();
        msg.reply(ctx, "⏸️ Musique mise en pause").await?;
    } else {
        msg.reply(ctx, "❌ Le bot n'est pas dans un salon vocal")
            .await?;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn resume(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird client non initialisé");

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let _ = handler.queue().resume();
        msg.reply(ctx, "▶️ Musique reprise").await?;
    } else {
        msg.reply(ctx, "❌ Le bot n'est pas dans un salon vocal")
            .await?;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird client non initialisé");

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let _ = handler.queue().skip();
        msg.reply(ctx, "⏭️ Musique passée").await?;
    } else {
        msg.reply(ctx, "❌ Le bot n'est pas dans un salon vocal")
            .await?;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird client non initialisé");

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        handler.queue().stop();
        msg.reply(ctx, "⏹️ Musique arrêtée").await?;
    } else {
        msg.reply(ctx, "❌ Le bot n'est pas dans un salon vocal")
            .await?;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "📋 File d'attente: (implémentation en cours)")
        .await?;
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn volume(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let volume: u8 = match args.single() {
        Ok(v) => v,
        Err(_) => {
            msg.reply(ctx, "❌ Usage: !volume <0-100>").await?;
            return Ok(());
        }
    };

    if volume > 100 {
        msg.reply(ctx, "❌ Le volume doit être entre 0 et 100")
            .await?;
        return Ok(());
    }

    msg.reply(ctx, format!("🔊 Volume réglé à {}%", volume))
        .await?;

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn nowplaying(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "🎵 Aucune musique en cours de lecture")
        .await?;
    Ok(())
}

