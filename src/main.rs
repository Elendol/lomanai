use miette::{IntoDiagnostic, Result};
use reqwest::Client;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .user_agent(APP_USER_AGENT)
        .gzip(true)
        .build()
        .into_diagnostic()?;
    let form = reqwest::multipart::Form::new()
        .text("an", "Bos taurus")
        .text("an", "Homo sapiens")
        .text("an", "Mus musculus")
        .text("cmd", "Save as");

    let response = client
        .post("https://www.ncbi.nlm.nih.gov/Taxonomy/CommonTree/wwwcmt.cgi")
        .multipart(form)
        .send()
        .await
        .into_diagnostic()?;

    let body = response.text().await.into_diagnostic()?;
    println!("{}", &body);

    Ok(())
}
