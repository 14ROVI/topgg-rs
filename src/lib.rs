use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;
use governor::{Quota, RateLimiter, clock, state};

use futures_util::future;
use warp::Filter;
use futures::channel::mpsc;
use tokio::task;



const BASE_URL: &str = "https://top.gg/api";


/// This is the top.gg API client. It houses the functions needed to interact with their API.
pub struct Topgg {
    bot_id: u64,
    token: String,
    client: reqwest::Client,
    limiter: RateLimiter<state::direct::NotKeyed, state::InMemoryState, clock::DefaultClock>
}
impl Topgg {
    /// Returns a new client.
    /// 
    /// ## Arguments
    /// * `bot_id` - The ID of your bot
    /// * `token` - The top.gg token for that (or another valid) bot
    /// 
    /// ## Examples
    /// ```
    /// let client = topgg::Topgg::new(bot_id, token);
    /// // Do stuff with the client
    /// let votes = client.votes().await.unwrap();
    /// ```
    /// 
    pub fn new(bot_id: u64, token: String) -> Topgg {
        Topgg {
            bot_id: bot_id,
            token: token,
            client: reqwest::Client::new(),
            limiter: RateLimiter::direct(
                Quota::per_minute(NonZeroU32::new(60u32).unwrap())
            )
        }
    }


    /// A shortcut for getting the botinfo for your own bot.
    /// ## Examples
    /// ```
    /// let bot_info = client.my_bot().await.unwrap();
    /// ```
    pub async fn my_bot(&self) -> Option<Bot> {
        self.bot(self.bot_id).await
    }


    /// Gets the info for a bot given an ID. To get the info for your own bot `client.my_bot()` can be used as a shortcut.
    /// ## Examples
    /// ```
    /// let bot_info = lient.bot(668701133069352961).await.unwrap();
    /// ```
    pub async fn bot(&self, bot_id: u64) -> Option<Bot> {
        self.limiter.until_ready().await;
        println!("requesting");
        let url = format!("{}/bots/{}", BASE_URL, bot_id);
        let res = self.client
            .get(&url)
            .header("Authorization", &self.token)
            .send()
            .await;
        if res.is_err() {
            return None;
        }

        let res = res
            .unwrap()        
            .json::<JsonBot>()
            .await;
        if res.is_err() {
            return None;
        }
        let res = res.unwrap();

        Some( Bot {
            id: res.id.parse::<u64>().unwrap(),
            username: res.username,
            discriminator: res.discriminator,
            avatar: res.avatar,
            def_avatar: res.defAvatar,
            lib: res.lib,
            prefix: res.prefix,
            short_desc: res.shortdesc,
            long_desc: res.longdesc,
            tags: res.tags,
            website: res.website,
            support: res.support,
            github: res.github,
            owners: res.owners.into_iter().map(|u| u.parse::<u64>().unwrap()).collect(),
            guilds: res.guilds.into_iter().map(|u| u.parse::<u64>().unwrap()).collect(),
            invite: res.invite,
            date: res.date,
            certified_bot: res.certifiedBot,
            vanity: res.vanity,
            points: res.points,
            monthly_points: res.monthlyPoints,
            donate_bot_guild_id: res.donatebotguildid.parse::<u64>().ok()
        })
    }


    /// Gets the info for a user.
    /// ## Examples
    /// ```
    /// client.user(195512978634833920).await.unwrap();
    /// ```
    pub async fn user(&self, user_id: u64) -> Option<User> {
        self.limiter.until_ready().await;
        let url = format!("{}/users/{}", BASE_URL, user_id);
        let res = self.client
            .get(&url)
            .header("Authorization", &self.token)
            .send()
            .await;
        if res.is_err() {
            return None;
        }

        let res = res
            .unwrap()        
            .json::<JsonUser>()
            .await;
        if res.is_err() {
            return None;
        }
        let res = res.unwrap();

        Some( User {
            id: res.id.parse::<u64>().unwrap(),
            username: res.username,
            discriminator: res.discriminator,
            avatar:res.avatar,
            def_avatar: res.defAvatar,
            bio: res.bio,
            banner: res.banner,
            youtube: res.social.get("youtube").map(|r| r.parse::<String>().unwrap()),
            reddit: res.social.get("reddit").map(|r| r.parse::<String>().unwrap()),
            twitter: res.social.get("twitter").map(|r| r.parse::<String>().unwrap()),
            instagram: res.social.get("instagram").map(|r| r.parse::<String>().unwrap()),
            github: res.social.get("github").map(|r| r.parse::<String>().unwrap()),
            color: res.color,
            supporter: res.supporter,
            certified_dev: res.certifiedDev,
            moderator: res.r#mod,
            web_moderator: res.webMod,
            admin: res.admin,
        })
    }


    /// A shortcut for getting the votes for the bot that created the client.
    /// ## Examples
    /// ```
    /// let votes = client.my_votes().await.unwrap();
    /// ```
    pub async fn my_votes(&self) -> Option<Vec<u64>> {
        self.votes(self.bot_id).await
    }


    /// Gets the user IDs of all the users that have voted on the bot_id.
    /// ## Examples
    /// ```
    /// client.votes(668701133069352961).await.unwrap();
    /// ```
    pub async fn votes(&self, bot_id: u64) -> Option<Vec<u64>> {
        self.limiter.until_ready().await;
        let url = format!("{}/bots/{}/votes", BASE_URL, bot_id);
        let res = self.client
            .get(&url)
            .header("Authorization", &self.token)
            .send()
            .await;
        if res.is_err() {
            return None;
        }

        let res = res
            .unwrap()        
            .json::<Vec<PartialJsonUser>>()
            .await;
        if res.is_err() {
            return None;
        }
        let res = res.unwrap();

        Some(
            res.into_iter()
                .map(|u| u.id.parse::<u64>().unwrap())
                .collect()
        )
    }


    /// A shortcut for checking if a user has voted for your own bot.
    /// ## Examples
    /// ```
    /// let voted = client.voted_for_me(195512978634833920).await.unwrap();
    /// ```
    pub async fn voted_for_me(&self, user_id: u64) -> Option<bool> {
        self.voted(self.bot_id, user_id).await
    }


    /// Checks if a user has voted for the bot or not. Returns true if they have, false if they have not.
    /// ## Examples
    /// ```
    /// let voted = client.voted(668701133069352961, 195512978634833920)
    ///     .await
    ///     .unwrap();
    /// ```
    pub async fn voted(&self, bot_id: u64, user_id: u64) -> Option<bool> {
        self.limiter.until_ready().await;
        let url = format!("{}/bots/{}/check?userId={}", BASE_URL, bot_id, user_id);
        let res = self.client
            .get(&url)
            .header("Authorization", &self.token)
            .send()
            .await;
        if res.is_err() {
            return None;
        }

        let res = res
            .unwrap()        
            .json::<CheckVote>()
            .await;
        if res.is_err() {
            return None;
        }
        let res = res.unwrap();

        if res.voted == 0 {
            return Some(false);
        } else {
            return Some(true);
        }
    }


    /// A shortcut for getting the bot stats of the bot that created the client.
    /// ## Examples
    /// ```
    /// let stats = client.my_bot_stats().await.unwrap();
    /// ```
    pub async fn my_bot_stats(&self) -> Option<BotStats> {
        self.get_bot_stats(self.bot_id).await
    }


    /// Gets the 'stats' of the bot, this includes the server count, shard count, and shards (servers per shard).
    /// ## Examples
    /// ```
    /// client.get_bot_stats(Some(668701133069352961)).await.unwrap();
    /// ```
    pub async fn get_bot_stats(&self, bot_id: u64) -> Option<BotStats> {
        self.limiter.until_ready().await;
        let url = format!("{}/bots/{}/stats", BASE_URL, bot_id);
        let res = self.client
            .get(&url)
            .header("Authorization", &self.token)
            .send()
            .await;
        if res.is_err() {
            return None;
        }

        let res = res
            .unwrap()        
            .json::<BotStats>()
            .await;
        if res.is_err() {
            return None;
        }
        let res = res.unwrap();

        Some(res)
    }

    
    /// This posts the stats for your bot. Useful if you want to update the server count on your top.gg bot page. You can omit from having a `server_count` if you use `shards` where it is a Vec of the number of servers per shard. `shard_id` is only applicable if you use `sever_count` and it tells top.gg the number of servers for that indexed shard.
    /// ## Examples
    /// ```
    /// client.post_bot_stats(None, Some(vec![142, 532, 304]), None, None).await;
    /// client.post_bot_stats(Some(142), None, Some(0), None).await;
    /// client.post_bot_stats(Some(978), None, None, Some(3)).await;
    /// ```
    pub async fn post_bot_stats(
        &self,
        server_count: Option<u32>,
        shards: Option<Vec<u32>>,
        shard_id: Option<u32>,
        shard_count: Option<u32>
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.limiter.until_ready().await;
        let url = format!("{}/bots/{}/stats", BASE_URL, self.bot_id);
        self.client
            .post(&url)
            .header("Authorization", &self.token)
            .json(&PostBotStats {
                server_count: server_count,
                shards: shards,
                shard_id: shard_id,
                shard_count: shard_count,
            })
            .send()
            .await
    }
}



pub struct WebhookClient;
impl WebhookClient {
    /// Starts listening to a port and filtering requests with a authentication string.
    /// ## Examples
    /// ```rust
    /// use futures::StreamExt;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut events = topgg::WebhookClient::start(3030, "a-very-secret-password".to_string());
    ///     
    ///     while let Some(msg) = events.next().await {
    ///         println!("{:?}", msg)
    ///     }
    /// }
    /// ```
    pub fn start(port: u16, auth: String) -> mpsc::UnboundedReceiver<Webhook> {

        let filter = warp::header::<String>("authorization")
            .and_then(move |value| {
                if value == auth {
                    future::ok(())
                } else {
                    future::err(warp::reject::custom(Unauthorized))
                }
            })
            .untuple_one();

        let (event_send, event_read) = mpsc::unbounded();


        let webhook = warp::post()
            .and(filter)
            .and(warp::body::json())
            .map(move |hook: Webhook| {
                event_send.unbounded_send(hook).unwrap();
                warp::reply()
            });
        
        task::spawn(async move {
            warp::serve(webhook).run(([0, 0, 0, 0], port)).await;
        });
        
        event_read
    }
}



#[derive(Debug)]
struct Unauthorized;
impl warp::reject::Reject for Unauthorized {}
impl std::fmt::Display for Unauthorized {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Unauthorized")
    }
}
impl std::error::Error for Unauthorized {}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Webhook {
    pub bot: String,
    pub user: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub is_weekend: bool,
    pub query: Option<String>,
}




#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct JsonBot {
    id: String,
    username: String,
    discriminator: String,
    avatar: Option<String>,
    defAvatar: String, 
    lib: String,
    prefix: String,
    shortdesc: String,
    longdesc: Option<String>,
    tags: Vec<String>,
    website: Option<String>,
    support: Option<String>,
    github: Option<String>,
    owners: Vec<String>,
    guilds: Vec<String>,
    invite: Option<String>,
    date: String,
    certifiedBot: bool,
    vanity: Option<String>,
    points: u64,
    monthlyPoints: u64,
    donatebotguildid: String
}

#[derive(Deserialize, Debug)]
pub struct Bot {
    pub id: u64,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub def_avatar: String,
    pub lib: String,
    pub prefix: String,
    pub short_desc: String,
    pub long_desc: Option<String>,
    pub tags: Vec<String>,
    pub website: Option<String>,
    pub support: Option<String>,
    pub github: Option<String>,
    pub owners: Vec<u64>,
    pub guilds: Vec<u64>,
    pub invite: Option<String>,
    pub date: String,
    pub certified_bot: bool,
    pub vanity: Option<String>,
    pub points: u64,
    pub monthly_points: u64,
    pub donate_bot_guild_id: Option<u64>
}


#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct JsonUser {
    id: String,
    username: String,
    discriminator: String,
    avatar: Option<String>,
    defAvatar: String,
    bio: Option<String>,
    banner: Option<String>,
    social: HashMap<String, String>,
    color: Option<String>,
    supporter: bool,
    certifiedDev: bool,
    r#mod: bool,
    webMod: bool,
    admin: bool,
}

#[derive(Debug)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
    pub def_avatar: String,
    pub bio: Option<String>,
    pub banner: Option<String>,
    pub youtube: Option<String>,
    pub reddit: Option<String>,
    pub twitter: Option<String>,
    pub instagram: Option<String>,
    pub github: Option<String>, 
    pub color: Option<String>,
    pub supporter: bool,
    pub certified_dev: bool,
    pub moderator: bool,
    pub web_moderator: bool,
    pub admin: bool,
}


#[derive(Deserialize, Debug)]
struct PartialJsonUser {
    id: String,
    username: String,
    discriminator: String,
    avatar: Option<String>
}

#[derive(Debug)]
pub struct PartialUser {
    pub id: u64,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>
}


#[derive(Deserialize, Debug)]
struct CheckVote {
    voted: i8
}


#[derive(Deserialize, Debug)]
pub struct BotStats {
    pub server_count: Option<u32>,
    pub shards: Vec<u32>,
    pub shard_count: Option<u32>
}


#[derive(Serialize, Debug)]
struct PostBotStats {
    server_count: Option<u32>,
    shards: Option<Vec<u32>>,
    shard_id: Option<u32>,
    shard_count: Option<u32>,
}