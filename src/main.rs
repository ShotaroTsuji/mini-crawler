use reqwest::blocking::ClientBuilder;
use url::Url;
use mini_crawler::LinkExtractor;
use mini_crawler::crawler::Crawler;
use std::time::Duration;
use structopt::StructOpt;

/// A toy web crawler
#[derive(StructOpt)]
struct Opt {
    /// Maximum number of pages to be crawled
    #[structopt(short="n")]
    maximum_pages: usize,
    /// URL where this program starts crawling
    start_page: Url,
}

fn main() -> eyre::Result<()> {
    env_logger::init();

    let opt = Opt::from_args();

    let client = ClientBuilder::new()
        .build()?;
    let extractor = LinkExtractor::from_client(client);

    let crawler = Crawler::new(&extractor, opt.start_page);

    let wait = Duration::from_millis(100);

    for url in crawler.take(opt.maximum_pages) {
        println!("{}", url);
        std::thread::sleep(wait.clone());
    }

    Ok(())
}
