use std::env::var;

use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, from_str};

pub struct GithubApi {
    client: reqwest::Client,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(transparent)]
pub struct Repos {
    pub repos: Vec<Repo>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Repo {
    pub name: String,
    pub url: String,
    pub full_name: String,
}
pub struct Languages {
    pub languages: Map<String, Value>,
}

impl GithubApi {
    fn headers(&self) -> HeaderMap {
        let key: String = var("GITHUB_KEY").expect("GITHUB_KEY environment variable");
        let token = format!("Bearer {}", key);
        let mut authorization = HeaderValue::from_str(&token).expect("Header value invalid");
        authorization.set_sensitive(true);

        let mut map = HeaderMap::new();
        map.insert("User-agent", "super-computing-machine/1.0".parse().unwrap());
        map.insert("Accept", "application/vnd.github+json".parse().unwrap());
        map.insert("Authorization", authorization);
        map.insert("X-GitHub-Api-Version", "2022-11-28".parse().unwrap());
        map
    }

    pub async fn user_repos(&self, user: &str) -> Option<Repos> {
        let req = self
            .client
            .get(format!(
                "https://api.github.com/users/{user}/repos?per_page=100"
            ))
            .headers(self.headers());

        println!("Request: {:?}", req);

        let req_res = req.send().await.unwrap().text().await.unwrap();
        serde_json::from_str(&req_res.to_string()).ok()
    }

    pub async fn repo_languages(&self, user: &str, repo: &str) -> Option<Languages> {
        let req = self
            .client
            .get(format!(
                "https://api.github.com/repos/{user}/{repo}/languages"
            ))
            .headers(self.headers());
        let res = req.send().await.unwrap().text().await.unwrap();
        let parsed: Value = from_str(&res).unwrap();
        let languages: Map<String, Value> = parsed.as_object().unwrap().clone();

        Some(Languages { languages })
    }
}

pub fn new_client() -> Option<GithubApi> {
    let client = reqwest::Client::builder().build().expect("builder failed");

    Some(GithubApi { client })
}
