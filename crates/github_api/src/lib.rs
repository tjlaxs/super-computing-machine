use std::collections::HashMap;
use std::env::var;

pub struct GithubApi {
    client: reqwest::Client,
}

impl GithubApi {
    pub async fn repo_languages(&self) -> Option<String> {
        let key: String = var("GITHUB_KEY").ok()?;
        let token = format!("Bearer {}", key);

        let mut map = HashMap::new();
        map.insert("Accept", "application/vnd.github+json");
        map.insert("Authorization", &key);
        map.insert("X-GitHub-Api-Version", "2022-11-28");

        println!("{}", token);

        let res = self
            .client
            .post("https://api.github.com/repos/tjlaxs/super-duper-parakeet/contents/README.md")
            .json(&map)
            .send()
            .await;

        Some(format!("{:?}", res))
    }
}

pub fn new_client() -> GithubApi {
    GithubApi {
        client: reqwest::Client::new(),
    }
}
