/*
    This file will embed metadata into the .flac audio files.
 */
use metaflac::{block::{PictureType::CoverFront, VorbisComment, StreamInfo}, Tag};
use crate::utils::json_structs::{ AlbumData, TrackItemInfo};

pub fn embed_metadata_flac (track : &String, meta_data_structs : &TrackItemInfo, image_bytes: Vec<u8>, album_struct: &AlbumData) -> () {
    let mut tag = Tag::read_from_path(track).expect("Unable to read FLAC metadata from file.");
    let mut vorbis_comments = VorbisComment::new();
    let mut streaming_info = StreamInfo::new();
    let tag_stream_info = tag.get_streaminfo().expect("Unable to find Stream Info.");
    println!("{}, {}\n",tag_stream_info.max_block_size, tag_stream_info.sample_rate);

    //set metadata
    tag.add_picture("image/jpeg", CoverFront, image_bytes);
    let album_artists: Vec<String> = meta_data_structs.performers
        .split("-")
        .map(|x| x.to_string())
        .collect();
    let artist: Vec<String> = [meta_data_structs.performer.name.clone()].to_vec();
    let title: Vec<String> = [meta_data_structs.title.clone()].to_vec();
    let album: Vec<String> = [album_struct.title.clone()].to_vec();

    vorbis_comments.set_album_artist(album_artists);
    vorbis_comments.set_artist(artist);
    vorbis_comments.set_title(title);
    vorbis_comments.set_genre(album_struct.genres_list.clone());
    vorbis_comments.set_track(meta_data_structs.track_number.try_into().expect("Unable to convert."));
    vorbis_comments.set_total_tracks(album_struct.tracks_count.try_into().expect("Unable to cast into u32"));
    vorbis_comments.set_album(album);
    tag.push_block(metaflac::Block::VorbisComment(vorbis_comments));

    tag.save().expect("unable to save tags.");
}