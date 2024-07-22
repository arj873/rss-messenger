use std::error::Error;
use rss::Channel;
use reqwest;
use tokio;


#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let mut currXml: String = "".to_string();
    match example_feed().await {
        Ok(channel) => currXml = channel.to_string(),
        Err(e) => eprintln!("Error: {}", e),
    }
    println!("{}", currXml);

}




async fn example_feed() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get("https://lorem-rss.herokuapp.com/feed?unit=second")
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

