# topgg-rs
A top.gg library.


## How to use
___
```rs
#[tokio::main]
async fn main() {
    topgg = topgg::Topgg::new(bot_id, topgg_token);
    
    // gets the top.gg info about your bot
    topgg.bot(None);
    
    // gets the top.gg info about another bot
    topgg.bot(Some(another_bot_id));
    
    // gets info about a user
    topgg.user(Some(a_user_id));
    
    // gets the IDs of the people who have voted for your bot (the id you initialised with)
    topgg.votes();
    
    // checks if a user has voted for the bot you initalised wiht
    topgg.voted(user_id);
    
    // gets stats about the server count, servers per shard, and shard count
    topgg.get_bot_stats(Some(another_bot_id));
    topgg.get_bot_stats(None); // or your bot
    
    // simply posts the server count to top.gg 
    topgg.post_bot_stats(Some(server_count), None, None, None);
    
    // It can also post more complex data like the servers per shard, shard_id of the server count, and shard count
    topgg.post_bot_stats(None, Some(shards), None, None);
    topgg.post_bot_stats(Some(server_count), None, Some(shard_id_that_posted), None);
    topgg.post_bot_stats(Some(server_count), None, None, Some(shard_count));
}
```

