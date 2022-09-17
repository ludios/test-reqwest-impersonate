use reqwest;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::builder()
        .build()
        .unwrap();

    for url in std::env::args().skip(1) {
	    let response = client.get(url).send().await?;
		dbg!(response);
	}

	Ok(())
}
