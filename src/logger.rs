use std::collections::HashMap;

use anyhow::Result;
use colored_json::ToColoredJson;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Request, Response,
};

fn log_headers(headers: &HeaderMap<HeaderValue>) -> Result<()> {
    let mut header_map = HashMap::<&str, &str>::new();

    for (k, v) in headers {
        header_map.insert(k.as_str(), v.to_str()?);
    }

    let json = serde_json::to_string(&header_map)?;

    println!("{}\n", json.to_colored_json_auto()?);

    Ok(())
}

pub fn log_request(req: Request, verbose: bool) -> Result<()> {
    if !verbose {
        return Ok(());
    }

    println!("Request Method: {:?}", req.method());
    println!("Request Headers: ");
    log_headers(req.headers())
}

pub async fn log_response(res: Response, verbose: bool) -> Result<()> {
    if verbose {
        println!("Response Status: {:?}", res.status());
        println!("Response Headers:");
        log_headers(res.headers())?;
    }

    let json = res.text().await?;

    println!("Response Body:");
    println!("{}", json.to_colored_json_auto()?);

    Ok(())
}
