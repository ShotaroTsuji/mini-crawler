use select::document::Document;
use select::predicate::Name;

fn main() -> eyre::Result<()> {
    let body = reqwest::blocking::get("https://www.rust-lang.org")?
        .text()?;
    let doc = Document::from(body.as_str());

    for href in doc.find(Name("a")).filter_map(|a| a.attr("href")) {
        println!("{}", href);
    }

    Ok(())
}
