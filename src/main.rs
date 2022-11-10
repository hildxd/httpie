use clap::Parser;

/// a naive implementation of the `httpie` command
#[derive(Parser, Debug)]
#[command(version = "1.0", author = "sunlee")]
struct Opt {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

/// feed get with an url
#[derive(Parser, Debug)]
struct Get {
    url: String,
}

#[derive(Parser, Debug)]
struct Post {
    url: String,
    body: Vec<String>,
}
fn main() {
    let opts = Opt::parse();
    println!("{:?}", opts);
}
