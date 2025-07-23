use reqwest::{ Error, Response };
use crate::utils::parser::*;

pub async fn program_entry(search_query : String) -> Result<String, Error>{
    if let Ok (get_music_request) = construct_url(&search_query, "get-music") {
        let res: Response = reqwest::get(get_music_request).await?;
        println!("Status: {}", res.status());
        println!("Headers:\n{:#?}", res.headers());

        let body = res.text().await?;
        println!("Body:\n{}", &body);

        parse_search_response2(&body, &search_query);

        let album_id = parse_search_response(&body, search_query);
        println!("\nAlbum Id: {}\n", &album_id);

        if let Ok (get_album_request) = construct_url(&album_id, "get-album") {
            let album_res: Response = reqwest::get(get_album_request).await?;
            let album_body = album_res.text().await?;
            println!("Album Body:\n{}", album_body);

            let track_ids: Vec<String> = get_track_ids(&album_body);
            for track in track_ids {
                println!("Track Id: {}\n", track);
            }
        } else {
            println!("Unable to find album body.\n");
        }
    } else {
        println!("Unable to process search request or no results were found.\n");
    }

    Ok("Program Executed Successfully!".to_string())
}