use chrono::Duration;
use gettextrs::gettext;

pub mod nightwave;
pub mod sortie;

/// 发送请求
pub async fn get_url(url: &str) -> String {
	reqwest::Client::builder()
		// 将所有流量代理到传递的URL
		.proxy(reqwest::Proxy::all(std::env::var("TELOXIDE_PROXY").unwrap()).unwrap())
		.build()
		.unwrap()
		.get(url)
		.send()
		.await
		.unwrap()
		.text()
		.await
		.unwrap()
}

/// 翻译星球名字
pub fn get_node(node: &str) -> String {
	let l = node.find('(').unwrap();
	let r = node.find(')').unwrap();
	format!("{}({})", &node[..l], gettext(&node[l + 1..r]))
}

/// 计算剩余时间
pub fn get_eta(expiry: &str) -> String {
	let expiry = chrono::DateTime::parse_from_rfc3339(expiry).unwrap().naive_utc();
	let local_time = chrono::Utc::now().naive_utc();
	let mut duration = expiry - local_time;

	let mut eta = String::new();
	let days = duration.num_days();
	if days > 0 {
		eta.push_str(format!("{} 天 ", days).as_str());
		duration = duration - Duration::days(days);
	}

	let hours = duration.num_hours();
	eta.push_str(format!("{} 时 ", hours).as_str());
	duration = duration - Duration::hours(hours);

	let minutes = duration.num_minutes();
	eta.push_str(format!("{} 分", minutes).as_str());

	eta
}
