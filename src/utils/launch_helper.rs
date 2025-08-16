use std::fs::{ File, OpenOptions };
use std::io::{ copy, Cursor };
use std::collections::HashMap;
use bytes::{ Bytes };
use metaflac::Tag;
use reqwest::{ Error, Response };

use crate::utils::json_structs::{ InnerAlbum, TrackItemInfo, UrlType };
use crate::utils::{ metadata_embed::embed_metadata_flac, parser::* };

pub async fn program_entry(search_query : String) -> Result<String, Error>{
    if let Ok (get_music_request) = construct_url(&search_query, UrlType::GetMusic) {
        let res: Response = reqwest::get(get_music_request).await?;
        println!("Status: {}", res.status());
        println!("Headers:\n{:#?}", res.headers());

        let body = res.text().await?;

        let (album_id, album_image_link) = parse_search_response(&body)
            .expect("Could not parse search response.\n");

        if let Ok (get_album_request) = construct_url(&album_id, UrlType::GetAlbum) {
            let album_res: Response = reqwest::get(get_album_request).await?;
            let album_body = album_res.text().await?;

            let album_data= get_track_ids(&album_body)
                .expect("Cannot fetch track ids for album.\n");
            let track_ids: &Vec<i32> = &album_data.data.track_ids;
            let track_metadata_list = &album_data.data.tracks;
            let mut track_map: HashMap<i32, &TrackItemInfo> = HashMap::new();
            for i in 0..track_ids.len() {
                track_map.insert(track_ids[i], &track_metadata_list.items[i]);
            }
            for (track_id, track_metadata) in &track_map{
                let _track_file = download_track(*track_id, *track_metadata, &album_image_link, &album_data)
                .await
                .expect("Unable to download file.");
            }
            
        } else {
            println!("Unable to find album body.\n");
        }
    } else {
        println!("Unable to process search request or no results were found.\n");
    }

    Ok("Program Executed Successfully!".to_string())
}

async fn download_track(track : i32, track_metadata : &TrackItemInfo, album_image_link : &String, album_data: &InnerAlbum) -> Result<(), reqwest::Error> {
    if let Ok(get_track_download_req) = construct_url(&track.to_string(), UrlType::DownloadMusic) {
        let track_download_res : Response = reqwest::get(get_track_download_req).await?;
        let track_body = track_download_res.text().await?;

        let track_download_link = get_dl_link(&track_body)
            .expect("Unable to find Download Link for track.");
        let track_download_res : Response = reqwest::get(track_download_link).await?;
        let track_name : &String = &track_metadata.title;
        let file_path = format!("./output/{track_name}.flac");
        let track_body = track_download_res.bytes().await
            .expect("Unable to download track.");

        let mut out = File::create(&file_path).expect("unable to create file.");
        copy(&mut Cursor::new(track_body), &mut out).expect("failed to copy.");
        let image_bytes = get_album_img(album_image_link).await?;
        
        embed_metadata_flac(&file_path, &track_metadata, image_bytes.to_vec(), &album_data.data); //TODO: figure out how to get metadata structs.
        let mut out = File::open(&file_path).expect("Unable to open file: ");
        let tag = Tag::read_from(&mut out).expect("unable to read tag from file: ");
        assert_eq!(tag.pictures().count(), 1);
    }
    println!("Track Id: {}\n", track);
    return Ok(());
}

async fn get_album_img(album_image_link : &String) -> Result<Bytes, reqwest::Error> {
    let album_image_res : Response = reqwest::get(album_image_link).await.expect("Unable to access image link.");
    let image_bytes = album_image_res.bytes().await.expect("Unable to get album image file.");
    return Ok(image_bytes);
}