use std::collections::HashMap;
use serde::{Deserialize, Serialize};


const BASE_URL: &str = "https://top.gg/api";


/// This is the top.gg API client. It houses the functions needed to interact with their API.
pub struct Topgg {
    bot_id: u64,
    token: String,
    client: reqwest::Client
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
    /// let votes = client.votes().await;
    /// ```
    /// 
    pub fn new(bot_id: u64, token: String) -> Topgg {
        Topgg {
            bot_id: bot_id,
            token: token,
            client: reqwest::Client::new()
        }
    }


    /// A shortcut for getting the botinfo for your own bot.
    pub async fn my_bot(&self) -> Bot {
        self.bot(None).await
    }


    /// Gets the info for a bot given an ID. If no ID is specified then it will use the one you used when setting up the client.
    /// ## Examples
    /// ```
    /// // Gets the info for the client's bot_id
    /// client.bot(None).await;
    /// // Gets info for a different bot
    /// client.bot(Some(668701133069352961)).await;
    /// ```
    pub async fn bot(&self, bot_id: Option<u64>) -> Bot {
        let bot_id = bot_id.unwrap_or(self.bot_id);
        let url = format!("{}/bots/{}", BASE_URL, bot_id);
        let res = self.client
            .get(&url)
            .header("Authorization", &self.token)
            .send()
            .await
            .unwrap()
            .json::<JsonBot>()
            .await
            .unwrap();

        Bot {
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
        }
    }


    /// Gets the info for a user.
    /// ## Examples
    /// ```
    /// client.user(195512978634833920).await;
    /// ```
    pub async fn user(&self, user_id: u64) -> User {
        let url = format!("{}/users/{}", BASE_URL, user_id);
        let res = self.client
            .get(&url)
            .header("Authorization", &self.token)
            .send()
            .await
            .unwrap()
            .json::<JsonUser>()
            .await
            .unwrap();

        User {
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
        }
    }


    /// Gets the user IDs of all the users that have voted on the bot_id used when creating the client.
    /// ## Examples
    /// ```
    /// client.votes().await;
    /// ```
    pub async fn votes(&self) -> Vec<u64> {
        let url = format!("{}/bots/{}/votes", BASE_URL, self.bot_id);
        let res = self.client
            .get(&url)
            .header("Authorization", &self.token)
            .send()
            .await
            .unwrap()
            .json::<Vec<PartialJsonUser>>()
            .await
            .unwrap();

        res.into_iter().map(|u| u.id.parse::<u64>().unwrap()).collect()
    }


    /// Checks if a user has voted for the bot or not. Returns true if they have, false if they have not.alloc
    /// ## Examples
    /// ```
    /// let voted = client.voted(195512978634833920).await;
    /// ```
    pub async fn voted(&self, user_id: u64) -> bool {
        let url = format!("{}/bots/{}/check?userId={}", BASE_URL, self.bot_id, user_id);
        let res = self.client
            .get(&url)
            .header("Authorization", &self.token)
            .send()
            .await
            .unwrap()
            .json::<CheckVote>()
            .await
            .unwrap();

        if res.voted == 0 {
            return false;
        } else {
            return true;
        }
    }


    /// Gets the 'stats' of the bot, this includes the server count, shard count, and shards (servers per shard).
    /// ## Examples
    /// ```
    /// // Gets the stats for your own bot
    /// client.get_bot_stats(None).await
    /// // Gets the stats for another bot
    /// client.get_bot_stats(Some(668701133069352961)).await
    /// ```
    pub async fn get_bot_stats(&self, bot_id: Option<u64>) -> BotStats {
        let bot_id = bot_id.unwrap_or(self.bot_id);
        let url = format!("{}/bots/{}/stats", BASE_URL, bot_id);
        let res = self.client
            .get(&url)
            .header("Authorization", &self.token)
            .send()
            .await
            .unwrap()
            .json::<BotStats>()
            .await
            .unwrap();

        res
    }

    
    /// This posts the stats for your bot. Useful if you want to update the server count on your top.gg bot page. You can omit from having a `server_count` if you use `shards` where it is a Vec of the number of servers per shard. `shard_id` is only applicable if you use `sever_count` and it tells top.gg the number of servers for that indexed shard.
    /// ## Examples
    /// ```
    /// client.post_bot_stats(None, Some(vec![142, 532, 304]), None, None).await;
    /// client.post_bot_stats(Some(142), None, Some(0), None).await;
    /// client.post_bot_stats(Some(978), None, None, Some(3)).await;
    /// ```
    pub async fn post_bot_stats(&self, server_count: Option<u32>, shards: Option<Vec<u32>>, shard_id: Option<u32>, shard_count: Option<u32>) {
        if server_count.is_none() && shards.is_none() {
            return;
        }

        let url = format!("{}/bots/{}/stats", BASE_URL, self.bot_id);
        let res = self.client
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
            .unwrap();

        println!("{:#?}", res)
    }
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