use serde_json::{ from_str };
use substring::Substring;
use crate::utils::json_structs::SearchResult;
/*
File to parse all the user inputs and crafts them into usable strings.
*/
pub fn construct_url(input : &String, url_type : &str) -> Result<String, String> {
    let domain_string = String::from("https://us.qobuz.squid.wtf/api/");
    let get_music_string = String::from("get-music?q=");

    let get_album_string = String::from("get-album?album_id=");

    let download_tracks = String::from("download-music?track_id=");
    let track_quality = String::from("&quality=27");

    match url_type {
        "get-music"      => { Ok(format!("{domain_string}{get_music_string}{input}&offset=0")) }
        "get-album"      => { Ok(format!("{domain_string}{get_album_string}{input}")) }
        "download-music" => { Ok(format!("{domain_string}{download_tracks}{input}{track_quality}")) }
        _                => { Ok(format!("{domain_string}"))}
    }
}

/*
Takes the body of the response 
and returns a string for the specific search song/album/artist the user had searched for.
 */

pub fn parse_search_response(body : &String) -> Result<String, serde_json::Error> {
    let body_json : SearchResult = from_str(body).expect("Could not map JSON onto struct.");

    let album_id  = &body_json.data.albums.items[0].id;

    return Ok(format!("{album_id}"));
}

/*
    Takes the an album id and returns the track ids within as a string Vector
 */
pub fn get_track_ids(album_body : &String) -> Vec<String> {

    let album_split = album_body.split("\"track_ids\"").collect::<Vec<_>>();
    let album_tracks_raw_string = if album_split.len() > 1 { album_split[1].to_string()} else {album_split[0].to_string()};
    println!("Album_tracks_raw_string: {}\n",album_tracks_raw_string);

    let index_touple = chunk_str(&album_tracks_raw_string, ':', 2);
    let start_index = index_touple.0 + 1;
    let end_index = index_touple.1;
    let track_raw_string = album_tracks_raw_string.substring(start_index.try_into().unwrap(), end_index.try_into().unwrap()).to_string();

    let index_touple = chunk_str_diff(&track_raw_string.to_string(), '[', ']');
    let track_string = track_raw_string.substring((index_touple.0 + 1).try_into().unwrap(), index_touple.1.try_into().unwrap());

    let track_ids =  track_string.split(',').map(|s| s.to_string()).collect::<Vec<String>>();

    return track_ids;
}

/*
    chunks strings in terms of "\ characters and returns a touple of two i32s marking the start index and end index.
 */
fn chunk_str(str : &String, to_remove : char, times_seen : i32) -> (i32, i32) {
    let char_vec : Vec<char> = str.chars().collect();
    let start    : i32 = 0;
    let mut end  : i32 = 0;
    let mut i    : i32 = 0; 
    let mut seen : i32 = 0;

    for ch in char_vec{

        if ch == (to_remove) { 
            seen += 1;
        }

        if seen == times_seen {
            end = i;
            break; 
        }
        i += 1;
    }
    // start offset to remove quotation.
    return (start, end);
}
/*
    to chunk strings by defining a start char and an end char.
 */
fn chunk_str_diff (str : &String, start : char, end : char) -> (i32, i32) {
    let char_vec       : Vec<char> = str.chars().collect();
    let mut start_index: i32 = 0;
    let mut end_index  : i32 = 0;
    let mut i          : i32 = 0; 

    for ch in char_vec{

        if ch == (start) { 
            start_index = i;
        }

        if ch == (end) {
            end_index = i;
        }

        i += 1;
    }
    return (start_index, end_index);
}