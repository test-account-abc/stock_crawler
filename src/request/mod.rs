use std::error::Error;

use hyper::{
    body::{Bytes, HttpBody},
    Client, Uri,
};
use hyper_tls::HttpsConnector;

pub async fn get_request(url: String) -> Result<String, Box<dyn Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let uri = url.parse::<Uri>()?;
    let mut response = client.get(uri).await?;
    if !response.status().is_success() {
        panic!("Error");
    }
    let mut body = String::new();
    while let Some(next) = response.body_mut().data().await {
        let chunk = next?;
        let str = convert_to_str(&chunk);
        match str {
            Ok(str) => body.push_str(&str),
            Err(error) => {
                println!("{}", error)
            }
        }
    }
    Ok(body)
}

fn convert_to_str(chunk: &Bytes) -> Result<String, Box<dyn Error>> {
    let result = std::str::from_utf8(&chunk);
    match result {
        Ok(result) => Ok(result.to_owned()),
        Err(error) => Err(Box::new(error)),
    }
}
