use clap::Parser;
use tiktok_downloader::tiktok::common::TiktokResource;
use tiktok_downloader::util::error::Error;
use tiktok_downloader::util::io::Download;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    url: String,
}

// https://v.douyin.com/UXCsJfR/
#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    let tiktok = TiktokResource::new(&args.url).await;
    if let Some(tiktok) = tiktok {
        match tiktok {
            TiktokResource::Image(image) => {
                let path = image.get_download_folder();
                image.download(path).await;
            }
            _ => {}
        }
    }
    Ok(())
}
