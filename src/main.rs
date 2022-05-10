use core::fmt;
use std;
mod cli;
mod query;
use serde::Deserialize;

#[derive(Deserialize)]
struct Document {
    title: String,
}
#[derive(Deserialize)]
#[allow(non_snake_case)]
struct LibraryResult {
    numFound: u16,
    docs: Vec<Document>,
}

impl fmt::Debug for LibraryResult {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        println!("{}", self.numFound);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = cli::read_args().unwrap();

    let url = query::make_url(query);
    let resp = reqwest::get(url).await?.json::<LibraryResult>().await?;

    println!("{} results.", resp.numFound);

    if resp.numFound > 10 {
        for index in 0..3 {
            println!("#{} {}", index + 1, resp.docs[0].title);
        }
        println!("...");
        println!("#{} {}", resp.numFound, resp.docs.last().unwrap().title);
    }

    Ok(())
}
