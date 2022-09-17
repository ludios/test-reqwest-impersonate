use reqwest_impersonate;
use reqwest_impersonate::browser::ChromeVersion;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest_impersonate::Client::builder()
        .chrome_builder(ChromeVersion::V104) // same result
        .build()
        .unwrap();

    for url in std::env::args().skip(1) {
	    let response = client.get(url).send().await?;
		dbg!(response);
	}

	Ok(())
}
