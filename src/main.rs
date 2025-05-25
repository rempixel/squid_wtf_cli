mod parser;
use std::{io, sync::Arc};
use parser::get_track_ids;
use reqwest::Response;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    println!("Search for your album, artist, or track!");

    println!("Please enter your query:");
    loop {
        let mut search_query = String::new();
        
        io::stdin()
            .read_line(&mut search_query)
            .expect("Failed to read user query.");
        //craft first search query

        let get_music_request = parser::construct_url(&search_query, "get-music");
  
        let res: Response = reqwest::get(get_music_request).await?;
        println!("Status: {}", res.status());
        println!("Headers:\n{:#?}", res.headers());

        let body = res.text().await?;
        println!("Body:\n{}", body);

        let album_id = parser::parse_search_response(&body, search_query);
        println!("\nAlbum Id: {}\n", &album_id);
        
        let get_album_request = parser::construct_url(&album_id, "get-album");
        
        let res : Response = reqwest::get(get_album_request).await?;

        let album_body = res.text().await?;
        println!("Album Body:\n{}", album_body);

        let track_ids = get_track_ids(&album_body);
        for track in track_ids {
            println!("Track Id: {}\n", track);
            
        }
        
        break;
    }
    /* 
    let res = reqwest::get("https://us.qobuz.squid.wtf/api/download-music?track_id=279295184&quality=27").await?;
        println!("Status: {}", res.status());
        println!("Headers:\n{:#?}", res.headers());

        let body = res.text().await?;
        println!("Body:\n{}", body);
    */
    
    Ok(())
}
