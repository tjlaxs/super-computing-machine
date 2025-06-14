use github_api::new_client;
use std::{thread, time};

#[tokio::main]
async fn main() {
    let client = new_client().expect("no client");
    let repos = client.user_repos("tjlaxs").await.expect("failing");
    for r in repos.repos {
        thread::sleep(time::Duration::from_millis(50));
        let languages = client
            .repo_languages("tjlaxs", &r.name)
            .await
            .expect("repo langs");
        println!("{}", r.name);
        for (lang, num) in languages.languages {
            println!("  {}: {}", lang, num)
        }
    }
}
