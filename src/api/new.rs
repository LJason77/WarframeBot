//! 新闻

pub async fn get_new() -> String {
	let json = reqwest::Client::builder()
		// 将所有流量代理到传递的URL
		.proxy(reqwest::Proxy::all(std::env::var("TELOXIDE_PROXY").unwrap()).unwrap())
		.build()
		.unwrap()
		.get("https://api.warframestat.us/pc/news")
		.header("Accept-Language", "zh")
		.send()
		.await
		.unwrap()
		.text()
		.await
		.unwrap();
	let news: Vec<crate::models::new::New> = serde_json::from_str(&json).unwrap();

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
