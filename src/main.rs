use github_api::repo_languages;

fn main() {
    let res = repo_languages();

    println!("Response:\n{:?}", res);
}
