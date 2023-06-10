use crate::util::error::Error;
use async_trait::async_trait;
use futures_util::StreamExt;
use http::{
    header::{CACHE_CONTROL, COOKIE, HOST, LOCATION, USER_AGENT},
    HeaderMap, HeaderValue,
};
use reqwest::{self, Response};
use std::io::Write;
use std::{
    fs::{create_dir, File},
    path::PathBuf,
};
use url::Url;

pub async fn download_file(url: &str, path: &PathBuf, filename: String) -> Result<(), Error> {
    let client = reqwest::Client::builder().build().unwrap();
    let resp = client.get(url).send().await.expect("get url error!");
    let mut dest_file = {
        let suffer = resp
            .url()
            .path()
            .split("/")
            .last()
            .and_then(|name| name.split(":").last())
            .and_then(|name| name.split(".").last())
            .unwrap_or("");
        let filename = if suffer != "" {
            format!("{}.{}", filename, suffer)
        } else {
            filename.to_string()
        };
        if !path.exists() {
            create_dir(&path).expect("create dir error!");
        }
        let fname = path.join(filename);
        File::create(&fname.as_path()).expect("create file error!")
    };
    let mut stream = resp.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(format!("Error while downloading file")))?;
        dest_file
            .write_all(&chunk)
            .or(Err(format!("Error while writing to file")))?;
    }
    Ok(())
}

pub fn write_text_to_file(text: &str, filename: &str) {
    let cur_dir = std::env::current_dir().unwrap();
    let mut dest_file = {
        let fname = cur_dir.join(filename);
        File::create(fname.as_path()).expect("create file error!")
    };
    dest_file.write(text.as_bytes()).expect("write file error!");
}
/**
 * 自动重定向请求，获取Response
 */
pub async fn get_location(url: &str) -> Option<Response> {
    let mut stop = false;
    let mut req_url = url.to_string();
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let mut response = Option::None;
    let mut common_headers = HeaderMap::new();
    common_headers.insert(USER_AGENT, HeaderValue::from_str(url).unwrap());
    common_headers.insert(CACHE_CONTROL, HeaderValue::from_str("no-cache").unwrap());
    common_headers.insert(COOKIE, HeaderValue::from_str("__ac_nonce=0647c28a400fb727d45d; __ac_signature=_02B4Z6wo00f01Gmm06QAAIDBCqwTzytOaGhphtcAAH4d77; __ac_referer=__ac_blank").unwrap());
    while !stop {
        let host = Url::parse(&req_url)
            .expect("parse url error!")
            .host_str()
            .unwrap()
            .to_string();
        let mut headers = common_headers.clone();
        headers.insert(HOST, HeaderValue::from_str(&host).unwrap());
        response = Some(client.get(&req_url).headers(headers).send().await.unwrap());
        match response.as_ref().unwrap().headers().get(LOCATION) {
            Some(location) => {
                req_url = location.to_str().unwrap().to_string();
            }
            None => {
                stop = true;
            }
        }
    }
    response
}

#[async_trait]
pub trait Download {
    async fn download(&self, path: PathBuf);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tiktok::image::parse_image_url;

    #[tokio::test]
    async fn test_get_location() {
        if let Some(resp) = get_location("https://v.douyin.com/UXCsJfR/").await {
            println!("url: {}", resp.url());
            let body = resp.text().await.unwrap();
            let urls = parse_image_url(&body);
            println!("{}", urls.len());
        }
    }
}
