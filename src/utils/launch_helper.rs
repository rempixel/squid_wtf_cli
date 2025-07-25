use reqwest::{ Error, Response };
use crate::utils::json_structs::UrlType;
use crate::utils::parser::*;

pub async fn program_entry(search_query : String) -> Result<String, Error>{
    if let Ok (get_music_request) = construct_url(&search_query, UrlType::GetMusic) {
        let res: Response = reqwest::get(get_music_request).await?;
        println!("Status: {}", res.status());
        println!("Headers:\n{:#?}", res.headers());

        let body = res.text().await?;

        let album_id = parse_search_response(&body)
            .expect("Could not parse search response.\n");

        if let Ok (get_album_request) = construct_url(&album_id, UrlType::GetAlbum) {
            let album_res: Response = reqwest::get(get_album_request).await?;
            let album_body = album_res.text().await?;
            println!("Album Body:\n{}", album_body);

            let track_ids: Vec<i32> = get_track_ids(&album_body)
                .expect("Cannot fetch Track Ids for album.\n");
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