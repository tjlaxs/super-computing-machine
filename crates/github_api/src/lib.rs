use std::env::var;

use reqwest::header::{HeaderMap, HeaderValue};

pub struct GithubApi {
    client: reqwest::Client,
}

impl GithubApi {
    pub async fn repo_languages(&self) -> Option<String> {
        let key: String = var("GITHUB_KEY").expect("GITHUB_KEY environment variable");
        let token = format!("Bearer {}", key);
        let mut authorization = HeaderValue::from_str(&token).expect("Header value invalid");
        authorization.set_sensitive(true);

        let mut map = HeaderMap::new();
        map.insert("User-agent", "super-computing-machine/1.0".parse().unwrap());
        map.insert("Accept", "application/vnd.github+json".parse().unwrap());
        map.insert("Authorization", authorization);
        map.insert("X-GitHub-Api-Version", "2022-11-28".parse().unwrap());

        let req = self
            .client
            .get("https://api.github.com/repos/tjlaxs/super-duper-parakeet/languages")
            .headers(map);

        println!("Request: {:?}", req);

        let res = req.send().await.unwrap().text().await.unwrap();

        Some(res.to_string())
    }
}

pub fn new_client() -> Option<GithubApi> {
    let client = reqwest::Client::builder().build().expect("builder failed");

    Some(GithubApi { client })
}
