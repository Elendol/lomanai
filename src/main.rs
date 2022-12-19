use clap::Parser;
use miette::{IntoDiagnostic, Result};
use reqwest::Client;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Binomial species name
    #[arg(short, long)]
    species: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Args::parse();

    let client = Client::builder()
        .user_agent(APP_USER_AGENT)
        .gzip(true)
        .build()
        .into_diagnostic()?;

    let mut form = reqwest::multipart::Form::new().text("cmd", "Save as");

    for species in cli.species {
        form = form.text("an", species);
    }

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
