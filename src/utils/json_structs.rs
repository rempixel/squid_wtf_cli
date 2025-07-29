use serde::{ Deserialize };

pub enum UrlType {
    GetMusic, 
    GetAlbum,
    DownloadMusic,
}
#[derive( Deserialize )]
pub struct SearchResult {
    pub data : Data,
}
#[derive( Deserialize )]
pub struct Data { 
    pub albums : Albums,
    pub tracks : Tracks,
    pub query : String,
}
#[derive( Deserialize )]
pub struct Albums {
    pub items : Vec<AlbumItemInfo>
}
#[derive( Deserialize )]
pub struct Artists {
   pub items : Vec<ArtistItemInfo>
}
#[derive( Deserialize )]
pub struct Tracks {
    pub items : Vec<TrackItemInfo>
}
#[derive ( Deserialize )]
pub struct AlbumItemInfo {
    pub id : String,
    pub release_date_original : String, 
    pub title : String,
    pub duration : i32,
    pub maximum_bit_depth : i32, 
    pub maximum_channel_count : i32,
    pub maximum_sampling_rate : f32,
    pub media_count : i32,
    pub tracks_count : i32,
    pub qobuz_id : i32,
    pub artist : AlbumArtistInfo,
    pub genre : AlbumGenreInfo,
    pub image : AlbumImages,
    pub label : LabelInfo,
}
#[derive(Deserialize)]
pub struct AlbumArtistInfo {
    pub name : String,
    pub albums_count : i32,
}
#[derive(Deserialize)]
pub struct AlbumGenreInfo {
    pub name : String,
}
#[derive(Deserialize)]
pub struct AlbumImages {
    pub large : String,
    pub small : String,
    pub thumbnail : String,
}
#[derive(Deserialize)]
pub struct LabelInfo { 
    pub name : String, 
    pub slug : String,
}
#[derive(Deserialize)]
pub struct ArtistItemInfo {

}
#[derive(Deserialize)]
pub struct TrackItemInfo {
    pub copyright : String,
    pub isrc : String,
    pub performers : String,
    pub release_date_original : String,
    pub title : String, 
    pub duration : i32,
    pub maximum_bit_depth : i32,
    pub maximum_channel_count : i32,
    pub maximum_sampling_rate : f32,
    pub track_number : i32,
    pub audio_info : TrackAudioInfo,
    pub composer : TrackComposerInfo,
    pub performer : TrackPerformerInfo,
  
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album : Option<AlbumItemInfo>,
}
#[derive(Deserialize)]
pub struct TrackAudioInfo {
    pub replaygain_track_gain : f32,
    pub replaygain_track_peak : f32,
}
#[derive(Deserialize)]
pub struct TrackComposerInfo {
    pub name : String,
}
#[derive(Deserialize)]
pub struct TrackPerformerInfo {
    pub name : String, 
}
#[derive(Deserialize)]
pub struct InnerAlbum { 
    pub data : AlbumData,
}
#[derive(Deserialize)]
pub struct AlbumData {
    pub artist : AlbumArtistInfo,
    pub composer : TrackComposerInfo,
    pub image : AlbumImages,
    pub label : LabelInfo,
    pub tracks : Tracks,
    pub track_ids : Vec<i32>
}
#[derive(Deserialize)]
pub struct DownloadMusic {
    pub data : DownloadData
}
#[derive(Deserialize)]
pub struct DownloadData { 
    pub url : String
}