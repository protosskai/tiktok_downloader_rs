use super::image::ImageResource;
use crate::util::io::get_location;

pub async fn get_real_domain(url: &str) -> Option<String> {
    if let Some(resp) = get_location(url).await {
        return Some(resp.url().to_string());
    }
    None
}

pub enum TiktokResource {
    Image(ImageResource),
}

impl TiktokResource {
    pub async fn new(url: &str) -> Option<Self> {
        let real_domain = if url.contains("v.douyin.com") {
            get_real_domain(&url).await.expect("get real domain error!")
        } else {
            url.to_string()
        };
        if real_domain.contains("note") {
            // 处理图片下载
            let image_resource = ImageResource::from_url(&real_domain).await;
            return Some(Self::Image(image_resource));
        }
        None
    }
}
