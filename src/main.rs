use github_api::new_client;

#[tokio::main]
async fn main() {
    let client = new_client();
    let res = client.repo_languages().await;

    println!("Response:\n{:?}", res);
}
