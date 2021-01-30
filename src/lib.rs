#![allow(unused)] 

mod discord_hook
{
    use std::collections::HashMap;
    use std::result;
    use serde_derive::Serialize;
    use serde_json::Result;
    use reqwest::blocking::Response;
    
    #[derive(Debug, Default, Serialize)]
    pub struct EmbedAutor {
        name: String,
        url: String,
        icon_url: String
    }

    #[derive(Debug, Default, Serialize)]
    pub struct EmbedField {
        name: String,
        value: String,
        inline: bool
    }

    #[derive(Debug, Default, Serialize)]
    pub struct EmbedFooter {
        text: String,
        icon_url: String,
    }

    #[derive(Debug, Default, Serialize)]
    pub struct EmbedImage {
        url: String
    }

    #[derive(Debug, Default, Serialize)]
    pub struct EmbedThumbnail {
        url: String
    }

    #[derive(Debug, Default, Serialize)]
    pub struct Embed {
        pub title: Option<String>,
        pub description: Option<String>,
        pub url: Option<String>,
        pub color: Option<u32>,
        pub fields: Option<Vec<EmbedField>>,
        pub autor: Option<EmbedAutor>,
        pub footer: Option<EmbedFooter>,
        pub timestamp: Option<String>,
        pub image: Option<EmbedImage>,
        pub thumbnail: Option<EmbedThumbnail>
    }


    #[derive(Debug, Default, Serialize)]
    struct AllowedMention {

    }


    #[derive(Debug, Default, Serialize)]
    struct DiscordHookPayload {
        #[serde(skip_serializing_if = "Option::is_none")] content: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")] username: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")] avatar_url: Option<String>,
        tts: bool,
        #[serde(skip_serializing_if = "Option::is_none")] embeds: Option<Vec<Embed>>
    }

    #[derive(Debug)]
    pub struct DiscordHook {
        webhook_url: String,
        client: reqwest::blocking::Client,
        payload: DiscordHookPayload,    
    }

    struct DefaultLength {
        content: usize,
        embeds: usize,
        username: usize,
        title: usize,
        description: usize,
        fields_name: usize,
        fields_value: usize,
        author_name: usize,
        footer_text: usize,
        images: usize
    }

    static MAX_LEN: DefaultLength = DefaultLength {
        content: 2000,
        embeds: 10,
        username: 256,
        title: 256,
        description: 2048,
        fields_name: 256,
        fields_value: 1024,
        author_name: 256,
        footer_text: 256,
        images: 4
    };

    impl DiscordHook {
        pub fn new(webhook_url: &str, content: &str) -> DiscordHook {
            let mut tmp_content = content.to_owned();
            tmp_content.truncate(MAX_LEN.content);
            
            let mut payload = DiscordHookPayload::default();
            payload.content = Some(tmp_content);

            DiscordHook {
                webhook_url: webhook_url.to_owned(),
                client: reqwest::blocking::Client::new(),
                payload: payload
            }
        }

        pub fn get_json(&self) -> String {
            serde_json::to_string_pretty(&self.payload).expect("Failed to build the json string.")
        }

        pub fn get_url(&self) -> &str {
            return &self.webhook_url;
        }

        pub fn fire(&self) -> reqwest::Result<Response> {
            self.client.post(self.get_url())
                .json(&self.payload)
                .send()
        }

        pub fn set_avatar_url(&mut self, avatar_url: &str) {
            self.payload.avatar_url = Some(avatar_url.to_owned());
        }

        pub fn set_username(&mut self, username: &str) {
            let mut tmp_username = username.to_owned();
            tmp_username.truncate(MAX_LEN.username);
            self.payload.username = Some(tmp_username);
        }

        pub fn add_embed(&mut self, embed: Embed) -> result::Result<(), String> {
            if self.count_ebmeds() > MAX_LEN.embeds {
                return Err(format!("To many embeded elements, maximum is {}", MAX_LEN.embeds));
            }

            if Option::is_none(&self.payload.embeds) {
                self.payload.embeds = Some(vec!());
            }
            self.payload.embeds.as_mut().unwrap().push(embed);
            Ok(())
        }

        pub fn new_with_embed(webhook_url: &str, embed: Embed) -> DiscordHook {
            let mut hook = DiscordHook {
                webhook_url: webhook_url.to_owned(),
                client: reqwest::blocking::Client::new(),
                payload: DiscordHookPayload::default()
            };

            hook.add_embed(embed);
            return hook;
        }

        pub fn count_ebmeds(&self) -> usize {
            if self.payload.embeds.is_none(){
                return 0;
            }
            self.payload.embeds.as_ref().unwrap().len()            
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::discord_hook::DiscordHook;
    use crate::discord_hook::Embed;
    use std::result;

    const test_hook_url: &str = "https://discord.com/api/webhooks/xxx/yyy";
    const test_avatar_url1: &str = "http://example.com/discord/avatars/Shabra2.jpg";
    const test_avatar_url2: &str = "http://example.com/discord/avatars/Shabra3.jpg";
    const test_avatar_url3: &str = "http://example.com/discord/avatars/Thorbard.jpg";

    #[test]
    fn it_works1() {
        let mut hook = DiscordHook::new(test_hook_url, "Test");
        hook.set_avatar_url(test_avatar_url2);
        hook.set_username("Umpalumpa3");

        let embed = Embed {
            title: Some("Schwimmen-Probe GESCHEITERT".to_owned()),
            description: Some("Talentwert: 3\nGE\t15\t[16]\nKO\t14\t[14]\nKK\t8\t[14]\nQS: -".to_owned()),
            color: Some(16711680),
            ..Default::default()
        };

        hook.add_embed(embed);

        let embed2 = Embed {
            title: Some("Schwimmen-Probe GELUNGEN".to_owned()),
            description: Some("Talentwert: 3\nGE\t15\t[16]\nKO\t14\t[14]\nKK\t 8\t[14]\nQS: -".to_owned()),
            color: Some(65280),
            ..Default::default()
        };

        hook.add_embed(embed2);

        hook.fire();
        println!("{:?}", hook.get_json());
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_works2() {
        let embed2 = Embed {
            title: Some("Schwimmen-Probe GELUNGEN".to_owned()),
            description: Some("Talentwert: 3\n```GE\t15\t[16]\nKO\t14\t[14]\nKK\t 8\t[14]```\nQS: -".to_owned()),
            color: Some(65280),
            ..Default::default()
        };

        let mut hook = DiscordHook::new_with_embed(test_hook_url, embed2);
        hook.set_avatar_url(test_avatar_url1);
        hook.set_username("Umpalumpa2");
        
        hook.fire();
        println!("{:?}", hook.get_json());
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_works3() {
        let embed2 = Embed {
            title: Some("Rumhibbel-Probe -3 __gelungen__".to_owned()),
            description: Some("Talentwert: 3\n```GE\t15\t[16]\nKO\t14\t[14]\nKK\t 8\t[14]```\n**QS: 2**".to_owned()),
            color: Some(65280),
            ..Default::default()
        };

        let mut hook = DiscordHook::new_with_embed(test_hook_url, embed2);
        hook.set_avatar_url(test_avatar_url3);
        hook.set_username("Thorbard");
        
        hook.fire();
        println!("{:?}", hook.get_json());
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_works4() {

        let mut hook = DiscordHook::new(test_hook_url, "some message");
        hook.set_avatar_url(test_avatar_url3);
        hook.set_username("Thorbärdel");
       
        let mut result: result::Result<(), String> = Err("bla".to_owned());
        for i in 0..11 {
            let embed = Embed {
                title: Some("Rumhibbel-Probe -3 __gelungen__".to_owned()),
                description: Some("Talentwert: 3\n```GE\t15\t[16]\nKO\t14\t[14]\nKK\t 8\t[14]```\n**QS: 2**".to_owned()),
                color: Some(65280),
                ..Default::default()
            };
            println!("loop counter {}", i, );
            
            result = hook.add_embed(embed);
            println!("embed counter {}", hook.count_ebmeds())
        }
        
               
        assert_eq!(result.is_err(), true);        
    }

    fn it_works5() {
        let mut hook = DiscordHook::new(test_hook_url, "Test");
        hook.set_avatar_url(test_avatar_url2);
        hook.set_username("Umpalumpa3");
      
        hook.fire();
        println!("{:?}", hook.get_json());
        assert_eq!(2 + 2, 4);
    }

}
