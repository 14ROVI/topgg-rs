# topgg-rs
A rust wrapper for the top.gg API that also abides by their rate limit.

## How to use
```
[dependencies]
topgg-rs = "0.2.0"
```

```rust
#[tokio::main]
async fn main() {
    c = topgg::Topgg::new(bot_id, topgg_token);
    
    // gets the top.gg info about your bot
    c.my_bot().await.unwrap();
    // gets the top.gg info about another bot
    c.bot(another_bot_id).await.unwrap();
    
    // gets info about a user
    c.user(a_user_id).await.unwrap();
    
    // gets the IDs of the people who have voted for your bot (the id you initialised with)
    c.my_votes().await.unrwap();
    // gets the IDs of the people who have voted for any bot
    c.votes(another_bot_id).await.unrwap();
    
    // checks if a user has voted for the bot you initalised with
    c.voted_for_me(user_id).await.unwrap();
    // checks if a user has voted for the bot
    c.voted(another_bot_id, user_id).await.unwrap();
    
    // gets stats about the server count, servers per shard, and shard count
    c.get_bot_stats(another_bot_id).await.unwrap();
    c.my_bot_stats().await.unwrap(); // or your bot
    
    // simply posts the server count to top.gg 
    c.post_bot_stats(Some(server_count), None, None, None).await;
    // It can also post more complex data like the servers per shard, shard_id of the server count, and shard count
    c.post_bot_stats(None, Some(shards), None, None).await;
    c.post_bot_stats(Some(server_count), None, Some(shard_id_that_posted), None).await;
    c.post_bot_stats(Some(server_count), None, None, Some(shard_count)).await;
}
```

