use std::io::Read;

pub fn repo_languages() -> Option<String> {
    let mut res = reqwest::blocking::get("http://httpbin.org/get").ok()?;
    let mut body = String::new();
    let _ = res.read_to_string(&mut body);
    Some(body)
}
