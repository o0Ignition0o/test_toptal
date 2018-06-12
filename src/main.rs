extern crate hyper;
extern crate hyper_native_tls;
extern crate scraper;
use std::env;

mod utils {
    use hyper::net::HttpsConnector;
    use hyper::{Client, Url};
    use hyper_native_tls::NativeTlsClient;
    use scraper::{Html, Selector};
    use std::io::Read;

    fn get_url(input: Option<String>) -> Url {
        match input {
            Some(url) => url.parse::<Url>().unwrap(),
            None => {
                panic!("Please provide me an http or https url as first argument !");
            }
        }
    }

    fn get_client_from_url(url: &Url) -> Client {
        match url.scheme() {
            "http" => Client::new(),
            "https" => {
                let ssl = NativeTlsClient::new().unwrap();
                let connector = HttpsConnector::new(ssl);
                Client::with_connector(connector)
            }
            _ => {
                panic!("The provided url is neither http nor https !");
            }
        }
    }

    pub fn get_links(input_url: Option<String>) -> Vec<String> {
        let url = get_url(input_url);
        let selector = Selector::parse("a").unwrap();
        let client = get_client_from_url(&url);

        let mut response = client.get(url).send().unwrap();
        let mut response_body = String::new();
        response.read_to_string(&mut response_body).unwrap();

        let document = Html::parse_document(&response_body);
        let mut matching_values: Vec<String> = Vec::new();
        for node in document.select(&selector) {
            matching_values.push(format!("{:?}", node.value().attr("href").unwrap()));
        }
        matching_values
    }
}
fn main() {
    let found_values = utils::get_links(env::args().nth(1));
    for value in &found_values {
        println!("{}", value);
    }
    println!("Found {} links total !", found_values.len());
}

#[cfg(test)]
mod tests {
    use utils;
    #[test]
    fn test_get_toptal_links() {
        let expected_links_count = 170; // For now :)
        let toptal_url = Some("https://www.toptal.com".to_string());
        let toptal_links = utils::get_links(toptal_url);
        assert_eq!(expected_links_count, toptal_links.len());
    }
}
