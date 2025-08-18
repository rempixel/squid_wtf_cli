/*
    This file will embed metadata into the .flac audio files.
 */
use metaflac::{block::{PictureType::CoverFront, VorbisComment}, Tag};
use crate::utils::json_structs::{ AlbumData, TrackItemInfo};

pub fn embed_metadata_flac (track : &String, meta_data_structs : &TrackItemInfo, image_bytes: Vec<u8>, album_struct: &AlbumData) -> () {
    let mut tag = Tag::read_from_path(track).expect("Unable to read FLAC metadata from file.");
    let mut vorbis_comments = VorbisComment::new();

    //set metadata
    tag.add_picture("image/jpeg", CoverFront, image_bytes);
    /* 
    Apparently iPod does not recognize this tag and if this is included it will mess up track order.
    Otherwise, this code works perfectly fine.
    let album_artists: Vec<String> = meta_data_structs.performers
        .split("-")
        .map(|x| x.to_string())
        .collect();
    */
    let artist: Vec<String> = [meta_data_structs.performer.name.clone()].to_vec();
    let title: Vec<String> = [meta_data_structs.title.clone()].to_vec();
    let album: Vec<String> = [album_struct.title.clone()].to_vec();
    let copyright: Vec<String> = [meta_data_structs.copyright.clone()].to_vec();
    let date: Vec<String> = [meta_data_structs.release_date_original.clone()].to_vec();
    let label: Vec<String> = [album_struct.label.name.clone()].to_vec();

    vorbis_comments.set_album_artist(artist.clone());
    vorbis_comments.set_artist(artist);
    vorbis_comments.set_title(title);
    vorbis_comments.set_genre(album_struct.genres_list.clone());
    vorbis_comments.set_track(meta_data_structs.track_number.try_into().expect("Unable to convert."));
    vorbis_comments.set_total_tracks(album_struct.tracks_count.try_into().expect("Unable to cast into u32"));
    vorbis_comments.set_album(album);
    vorbis_comments.set("label", label);
    vorbis_comments.set("copyright", copyright);
    vorbis_comments.set("date",date );
    //Print Metadata info.
    println!("Album Title: {}\n Album track#: {}\n", 
        vorbis_comments.title().unwrap()[0], vorbis_comments.track().unwrap());
    for genre in album_struct.genres_list.clone() {
        println!("Genre: {}\n", genre);
    }
   
    tag.push_block(metaflac::Block::VorbisComment(vorbis_comments));

    tag.save().expect("Unable to save tags to file.");
}