use reqwest::blocking::Client;
use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::Read;
use std::process::exit;

fn get_tag() -> String {
    if let Ok(tag) = env::var("INPUT_CREATED_TAG") {
        tag
    } else if let Ok(github_ref) = env::var("GITHUB_REF") {
        github_ref.split('/').last().unwrap_or("").to_string()
    } else {
        "".to_string()
    }
}

fn get_release_id(auth_header: &String, github_repository: &String, tag: &String) -> i64 {
    let client = Client::new();

    let url = format!(
        "https://api.github.com/repos/{}/releases/tags/{}",
        github_repository, tag
    );

    let response = client
        .get(&url)
        .header("Authorization", auth_header)
        .header("Accept", "*/*")
        .header("User-Agent", "Rust")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send();

    match response {
        Ok(res) => {
            if res.status().is_success() {
                let body: Value = res.json().expect("Github Release info should be a JSON");
                body["id"].as_i64().unwrap()
            } else {
                println!("here: {:?}", res);
                let res_status = res.status().clone();
                let res_body = res.text();
                println!(
                    "Response status: {}, body: {}",
                    res_status,
                    res_body.unwrap()
                );
                exit(1);
            }
        }
        Err(err) => {
            println!("{:?}", err);
            if let Some(response) = err.status() {
                let res_body = err.to_string();
                println!("Response status: {}, body: {}", response, res_body);
            } else {
                println!("is_body: {:?}", err.is_body());
                println!("is_decode: {:?}", err.is_decode());
                println!("is_status: {:?}", err.is_status());
                println!("is_builder: {:?}", err.is_builder());
                println!("is_connect: {:?}", err.is_connect());
                println!("is_request: {:?}", err.is_request());
                println!("is_timeout: {:?}", err.is_timeout());
                println!("is_redirect: {:?}", err.is_redirect());
            }
            exit(1);
        }
    }
}

fn read_asset(asset_path: &String) -> Vec<u8> {
    let mut file = File::open(asset_path).expect("Asset file not found");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    return buffer;
}

fn main() {
    let tag = get_tag();

    if tag.is_empty() {
        eprintln!("::error ::This is not a tagged push");
        exit(1);
    }

    let asset_file = env::var("ASSET_FILE").expect("ASSET_FILE not set");
    let asset_name = env::var("ASSET_NAME").unwrap_or(asset_file.clone());
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let github_repository = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
    let auth_header = format!("Bearer {}", github_token);

    let body = read_asset(&asset_file);

    let release_id = get_release_id(&auth_header, &github_repository, &tag);

    let client = Client::new();

    let url = format!(
        "https://uploads.github.com/repos/{}/releases/{}/assets?name={}",
        github_repository, release_id, asset_name
    );

    let response = client
        .post(&url)
        .header("Authorization", auth_header)
        .header("Accept", "*/*")
        .header("Content-Type", "application/zip")
        .header("User-Agent", "Rust")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .body(body)
        .send();

    match response {
        Ok(res) => {
            if res.status().is_success() {
                println!("::debug ::Uploaded: {}", asset_file);
                exit(0);
            }
            println!("::error:: {:?}", res);
            exit(1);
        }
        Err(err) => {
            if let Some(response) = err.status() {
                let res_body = err.to_string();
                println!(
                    "::error:: Response status: {}, body: {}",
                    response, res_body
                );
                exit(1);
            }
            println!("::error:: {:?}", err);
            exit(1);
        }
    };
}
