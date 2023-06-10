use std::path::PathBuf;

use crate::util::io::download_file;
use crate::util::io::Download;
use async_trait::async_trait;
use http::{
    header::{CACHE_CONTROL, COOKIE, HOST, USER_AGENT},
    HeaderMap, HeaderValue,
};
use scraper::{Html, Selector};
/**
 * 从html中解析出来图片的url
 */
pub fn parse_image_url(html_content: &str) -> Vec<String> {
    let doc = Html::parse_fragment(html_content);
    let selector = Selector::parse("main > div:nth-child(1) > div:nth-child(1) img").unwrap();
    let urls: Vec<String> = doc
        .select(&selector)
        .into_iter()
        .map(|ele| ele.value().attr("src").unwrap_or("").to_string())
        .filter(|ele| ele.len() != 0)
        .collect();
    urls
}
// head > title
pub fn parse_image_title(html_content: &str) -> String {
    let doc = Html::parse_fragment(html_content);
    let selector = Selector::parse("title").unwrap();
    let title = doc
        .select(&selector)
        .into_iter()
        .map(|e| e.text().collect::<Vec<_>>().concat())
        .next()
        .unwrap();
    title
}

pub async fn get_html_content(url: &str) -> String {
    let client = reqwest::Client::builder().build().unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_str(url).unwrap());
    headers.insert(HOST, HeaderValue::from_str("www.douyin.com").unwrap());
    headers.insert(CACHE_CONTROL, HeaderValue::from_str("no-cache").unwrap());
    headers.insert(COOKIE, HeaderValue::from_str("__ac_nonce=0647c28a400fb727d45d; __ac_signature=_02B4Z6wo00f01Gmm06QAAIDBCqwTzytOaGhphtcAAH4d77; __ac_referer=__ac_blank").unwrap());
    let resp = client.get(url).headers(headers).send().await.unwrap();
    let body = resp.text().await.unwrap();
    body
}

pub struct ImageResource {
    title: String,
    urls: Vec<String>,
}

impl ImageResource {
    pub fn new(title: &str, urls: Vec<String>) -> Self {
        Self {
            title: title.to_string(),
            urls,
        }
    }

    pub async fn from_url(url: &str) -> Self {
        let html_content = get_html_content(url).await;
        let title = parse_image_title(&html_content);
        let image_url_list = parse_image_url(&html_content);
        Self {
            title,
            urls: image_url_list,
        }
    }

    pub fn get_download_folder(&self) -> PathBuf {
        let cur_dir = std::env::current_dir().unwrap();
        cur_dir.join(&self.title)
    }
}
#[async_trait]
impl Download for ImageResource {
    async fn download(&self, path: std::path::PathBuf) {
        let mut number = 0;
        for url in &self.urls {
            download_file(url, &path, number.to_string())
                .await
                .expect("download file error!");
            number += 1;
        }
    }
}
