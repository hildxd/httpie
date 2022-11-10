use clap::Parser;
use anyhow::{ Result, anyhow };
use reqwest::Url;

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
    let url:Url = s.parse()?;
    Ok(url.into())
}
fn main() {
    let opts = Httpie::parse();
    println!("{:?}", opts);
}
