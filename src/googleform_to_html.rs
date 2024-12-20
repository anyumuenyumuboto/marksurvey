use dotenv::dotenv;
use html_builder::Buffer;
use html_builder::Html5;
use std::env;
use std::fmt::Write;

pub mod fetch_google_forms;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dbg!("googleform_to_html");
    dotenv().ok();

    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");
    let form_id = env::var("FORM_ID").expect("FORM_ID must be set");

    // build_html().unwrap();
    let access_token = fetch_google_forms::get_access_token(&client_id, &client_secret).await?;
    fetch_google_forms::fetch_google_form(&access_token, &form_id).await?;
    // get_google_forms().await?;
    Ok(())
}

fn build_html() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = Buffer::new(); // Contents added to buffer by each statement:
    let mut html = buf.html().attr("lang='en'"); // <html lang='en'>
    writeln!(html.head().title(), "Title!")?; // <head><title>Title!
    writeln!(html.body().h1(), "Header!")?; // </title></head><body><h1>Header!
    let html_string = buf.finish(); // </h1></body></html>
    dbg!("{}", html_string);
    Ok(())
}

async fn get_google_forms() -> Result<(), Box<dyn std::error::Error>> {
    dbg!("get_google_forms");
    let url = "https://www.rust-lang.org";
    let contents = reqwest::get(url).await?.text().await?;
    dbg!("{:?}", contents);
    Ok(())
}
