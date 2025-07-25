use serde_json::{ from_str };
use crate::utils::json_structs::{ SearchResult, UrlType, InnerAlbum };
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

pub fn parse_search_response(body : &String) -> Result<String, serde_json::Error> {
    let body_json : SearchResult = from_str(body)
        .expect("Could not map JSON onto struct.");
    let album_id  = &body_json.data.albums.items[0].id;

    return Ok(format!("{album_id}"));
}

/*
    Takes the an album id and returns the track ids within as a string Vector
 */
pub fn get_track_ids(album_body : &String) -> Result<Vec<i32>, String> {
    let album_struct : InnerAlbum = from_str(album_body)
        .expect("Unable to cast into album struct");
    let track_ids : Vec<i32> = album_struct.data.track_ids;
    
    return Ok(track_ids);
}