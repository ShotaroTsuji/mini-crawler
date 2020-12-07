pub mod crawler;

use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::Name;
use url::Url;
use url::ParseError as UrlParseError;
use thiserror::Error;

#[derive(Error,Debug)]
pub enum GetLinksError {
    #[error("Failed to send a request")]
    SendRequest(#[source] reqwest::Error),
    #[error("Failed to read the response body")]
    ResponseBody(#[source] reqwest::Error),
    #[error("Failed to make the link URL absolute")]
    AbsolutizeUrl(#[source] url::ParseError),
    #[error("Server returned an error")]
    ServerError(#[source] reqwest::Error),
}

pub struct LinkExtractor {
    client: Client,
}

impl LinkExtractor {
    pub fn from_client(client: Client) -> Self {
        Self {
            client: client,
        }
    }

    pub fn get_links(&self, url: Url) -> Result<Vec<Url>, GetLinksError> {
        log::info!("GET \"{}\"", url);
        let response = self.client.get(url).send()
            .map_err(|e| GetLinksError::SendRequest(e))?;
        let response = response.error_for_status()
            .map_err(|e| GetLinksError::ServerError(e))?;
        let base_url = response.url().clone();
        let status = response.status();
        let body = response.text()
            .map_err(|e| GetLinksError::ResponseBody(e))?;
        let doc = Document::from(body.as_str());
        let mut links = Vec::new();
        log::info!("Retrieved {} \"{}\"", status, base_url);

        for href in doc.find(Name("a")).filter_map(|a| a.attr("href")) {
            match Url::parse(href) {
                Ok(mut url) => {
                    url.set_fragment(None);
                    links.push(url);
                },
                Err(UrlParseError::RelativeUrlWithoutBase) => {
                    match base_url.join(href) {
                        Ok(mut url) => {
                            url.set_fragment(None);
                            links.push(url);
                        },
                        Err(e) => {
                            log::warn!("URL join error: {}", e);
                        },
                    }
                },
                Err(e) => {
                    log::warn!("URL parse error: {}", e);
                },
            }
        }

        Ok(links)
    }
}

impl crawler::AdjacentNodes for LinkExtractor {
    type Node = Url;

    fn adjacent_nodes(&self, v: &Self::Node) -> Vec<Self::Node> {
        let links = {
            let t0 = std::time::Instant::now();
            let links = self.get_links(v.clone());
            let elp = t0.elapsed();
            log::info!("GetLinks: {} ms", elp.as_millis());

            links
        };
        match links {
            Ok(links) => {
                log::info!("{} links found", links.len());
                links
            },
            Err(e) => {
                use std::error::Error;
                log::warn!("Error occurred: {}", e);

                let mut e = e.source();
                loop {
                    if let Some(err) = e {
                        log::warn!("Error source: {}", err);
                        e = err.source();
                    } else {
                        break;
                    }
                }

                vec![]
            },
        }
    }
}
