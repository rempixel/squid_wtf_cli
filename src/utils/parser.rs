use serde_json::{ from_str };
use crate::utils::json_structs::{ DownloadMusic, InnerAlbum, SearchResult, UrlType};
/*
File to parse all the user inputs and crafts them into usable strings.
*/
pub fn construct_url(input : &String, url_type : UrlType) -> Result<String, String> {
    let domain_string = String::from("https://us.qobuz.squid.wtf/api/");
    let get_music_string = String::from("get-music?q=");
    let get_album_string = String::from("get-album?album_id=");
    let download_tracks = String::from("download-music?track_id=");
    let track_quality = String::from("&quality=27");

    match url_type {
        UrlType::GetMusic      => { Ok(format!("{domain_string}{get_music_string}{input}&offset=0")) }
        UrlType::GetAlbum      => { Ok(format!("{domain_string}{get_album_string}{input}")) }
        UrlType::DownloadMusic => { Ok(format!("{domain_string}{download_tracks}{input}{track_quality}")) }
    }
}

/*
Takes the body of the response 
and returns a string for the specific search song/album/artist the user had searched for.
 */

pub fn parse_search_response(body : &String) -> Result<(String, String), serde_json::Error> {
    let body_json_result: Result<SearchResult, serde_json::Error> = from_str(body);

    let body_json : SearchResult = match body_json_result {
        Ok(structure) => structure, 
        Err(error) => panic!("Unable to map response into a structure: Error: {error:?}")
    };

    if let Some(album) = body_json.data.albums.items.get(0) {
        Ok((album.id.clone(), album.image.large.clone()))
    } else {
        Err(serde::de::Error::custom("No album found"))
    }
}


/*
    Takes the an album id and returns a tuple consisting the track ids within as a string Vector, and the Track Struct.
 */
pub fn get_track_ids(album_body : &String) -> Result<InnerAlbum, String> {
    let album_struct : InnerAlbum = from_str(album_body)
        .expect("Unable to cast into album struct");

    return Ok(album_struct);
}

/*
    Takes the response of a track body, parses into a struct and returns the url.
 */
pub fn get_dl_link(track_body : &String) -> Result<String, serde_json::Error> {
    let track_dl_struct : DownloadMusic = from_str(&track_body)
        .expect("Error Casting JSON onto struct.");

    return Ok(track_dl_struct.data.url);
}