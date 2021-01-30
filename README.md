# discord-webhook
Some rust library to use discord webhooks.   
**Be warned that this code is in alpha state!**

# Examples
Fire a text only hook:
```rust
let mut hook = DiscordHook::new(test_hook_url, "This is a simple text only webhook message.");
hook.set_avatar_url("https://example.com/some_image.jpg");
hook.set_username("Captain Hook");
hook.fire();
```

Fire a hook with embed:
```rust
let embed = Embed {
    title: Some("Schwimmen-Probe GESCHEITERT".to_owned()),
    description: Some("Talentwert: 3\nGE\t15\t[16]\nKO\t14\t[14]\nKK\t8\t[14]\nQS: -".to_owned()),
    color: Some(16711680),
    ..Default::default()
};

let mut hook = DiscordHook::new_with_embed(test_hook_url, embed);
hook.set_avatar_url(test_avatar_url2);
hook.set_username("Umpalumpa3");

let embed2 = Embed {
    title: Some("Schwimmen-Probe GELUNGEN".to_owned()),
    description: Some("Talentwert: 3\nGE\t15\t[16]\nKO\t14\t[14]\nKK\t 8\t[14]\nQS: -".to_owned()),
    color: Some(65280),
    ..Default::default()
};
hook.add_embed(embed2);

hook.fire();
```