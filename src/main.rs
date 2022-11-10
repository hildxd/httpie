use std::collections::HashMap;

use anyhow::{anyhow, Ok, Result};
use clap::Parser;
use mime::Mime;
use reqwest::{header, Client, Url, Response};

/// a naive implementation of the `httpie` command
#[derive(Parser, Debug)]
#[command(name = "httpie", bin_name = "httpie")]
#[command(version = "1.0", author = "sunlee")]
enum Httpie {
    Get(Get),
    Post(Post),
}

/// feed get with an url and we will retrieve the response for you
#[derive(Parser, Debug)]
struct Get {
    /// HTTP 请求的 URL
    #[arg(value_parser = parse_url)]
    url: String,
}

/// send post request
#[derive(Parser, Debug)]
struct Post {
    #[arg(value_parser = parse_url)]
    url: String,
    #[arg(value_parser = parse_kv_pair)]
    body: Vec<KvPair>,
}

#[derive(Debug, Clone)]
struct KvPair {
    key: String,
    value: String,
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    let mut iter = s.splitn(2, '=');
    let err = || anyhow!(format!("Failed to parse {}", s));
    let key = iter.next().ok_or_else(err)?;
    let value = iter.next().ok_or_else(err)?;
    Ok(KvPair {
        key: key.to_string(),
        value: value.to_string(),
    })
}

fn parse_url(s: &str) -> Result<String> {
    let url: Url = s.parse()?;
    Ok(url.into())
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    println!("{:?}", resp);
    Ok(())
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.key, &pair.value);
    }
    let resp = client
        .post(&args.url)
        .json(&body)
        .send()
        .await?;
    println!("{:?}", resp);
    Ok(())
}

async fn print_resp(resp: Response) -> Result<()> {
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    Ok(())
}

/// 将服务器返回的 content-type 解析成 Mime 类型
fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Httpie::parse();
    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("httpie"),
    );
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let result = match opts {
        Httpie::Get(ref args) => get(client, args).await?,
        Httpie::Post(ref args) => post(client, args).await?,
    };
    Ok(result)
}
