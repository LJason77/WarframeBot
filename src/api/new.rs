//! 新闻

use crate::models::{Header, New};

use super::get_url;

pub async fn get_new() -> String {
    let news =
        match get_url::<Vec<New>>("news", Some(Header { key: "Accept-Language", value: "zh" }))
            .await
        {
            Ok(news) => news,
            Err(err) => return err,
        };

    let mut news_str = String::new();
    for new in news.iter().rev() {
        news_str.push_str(
            format!(
                "<a href=\"{}\">{}</a>\n\n",
                new.link.replace("www.warframe.com", "www.warframe.com/zh-hans"),
                new.message
            )
            .as_str(),
        );
    }
    news_str
}
