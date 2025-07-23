use std::{io};
use crate::utils::launch_helper::program_entry;

pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Search for your album, artist, or track!");

    println!("Please enter your query:");

    loop {
        let mut search_query = String::new();
        
        io::stdin()
            .read_line(&mut search_query)
            .expect("Failed to read user query.");
        
        let search_query = search_query.trim().to_string();
        program_entry(search_query).await?;
        break;
    }
    Ok(())
}
