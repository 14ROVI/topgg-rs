# topgg-rs
A top.gg library.



## How to use
In your `Cargo.toml`
```
[dependencies]
topgg-rs = "0.1.0"
```

```rust
#[tokio::main]
async fn main() {
    c = topgg::Topgg::new(bot_id, topgg_token);
    
    // gets the top.gg info about your bot
    c.bot(None).await;
    
    // gets the top.gg info about another bot
    c.bot(Some(another_bot_id)).await;
    
    // gets info about a user
    c.user(Some(a_user_id)).await;
    
    // gets the IDs of the people who have voted for your bot (the id you initialised with)
    c.votes().await;
    
    // checks if a user has voted for the bot you initalised wiht
    c.voted(user_id).await;
    
    // gets stats about the server count, servers per shard, and shard count
    c.get_bot_stats(Some(another_bot_id)).await;
    c.get_bot_stats(None).await; // or your bot
    
    // simply posts the server count to top.gg 
    c.post_bot_stats(Some(server_count), None, None, None).await;
    
    // It can also post more complex data like the servers per shard, shard_id of the server count, and shard count
    c.post_bot_stats(None, Some(shards), None, None).await;
    c.post_bot_stats(Some(server_count), None, Some(shard_id_that_posted), None).await;
    c.post_bot_stats(Some(server_count), None, None, Some(shard_count)).await;
}
```

