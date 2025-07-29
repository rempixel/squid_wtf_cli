/*
    This file will embed metadata into the .flac audio files.
 */
use std::fs::File;
use metaflac::Tag;
use crate::utils::parser::construct_url;
use crate::utils::json_structs::UrlType;

pub fn embed_metadata_flac (track : File) -> () {
   let mut tag : Tag = Tag::new();
   
}