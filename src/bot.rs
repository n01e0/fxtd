use log::{error, info};
use regex::Regex;
use serenity::{
    async_trait,
    model::{
        channel::{GuildChannel, Message},
        gateway::Ready,
    },
    prelude::*,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        twitter(&ctx, &msg).await;
    }

    async fn thread_create(&self, ctx: Context, thread: GuildChannel) {
        if let Err(why) = thread.id.join_thread(&ctx.http).await {
            error!("Error joining thread: {:?}", why);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

async fn twitter(ctx: &Context, msg: &Message) {
    let url_regex =
        Regex::new(r"https?://(twitter.com|x.com)/[a-zA-Z0-9_]+/status/[0-9]+").unwrap();
    for url in url_regex.find_iter(&msg.content) {
        let host_regex = Regex::new(r"https?://(twitter.com|x.com)").unwrap();
        let new_url = host_regex.replace(url.as_str(), "https://fxtwitter.com");
        match msg.channel_id.say(&ctx.http, &new_url).await {
            Ok(_) => info!("Sent new link: {}", new_url),
            Err(why) => error!("Error sending message: {:?}", why),
        }
    }
}
