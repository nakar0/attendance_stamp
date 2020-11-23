use std::process;
use reqwest;
use tokio;
use scraper;


const SITE_URL: &str = "https://attendance.moneyforward.com/my_page";

#[tokio::main]
async fn main() {
    panic!("yet");
}

async fn fetch_site() -> reqwest::Response {
     match reqwest::get(SITE_URL).await {
        Ok(res)=> res,
        Err(err)=> {
            println!("{}", err);
            process::exit(1);
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fetch_site_body() {
        let res = fetch_site().await;
        let body = res.text().await.unwrap();
        assert_eq!(body.contains("<title>ログイン | マネーフォワード クラウド勤怠</title>"), true);
    }
}