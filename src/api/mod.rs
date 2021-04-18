use gettextrs::gettext;

pub mod sortie;

/// 发送请求
pub async fn get_url(url: &str) -> String {
	reqwest::Client::builder()
		// 将所有流量代理到传递的URL
		.proxy(reqwest::Proxy::all(env!("TELOXIDE_PROXY")).unwrap())
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
