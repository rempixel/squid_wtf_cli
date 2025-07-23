use serde::{ Deserialize };
use std::{ collections::HashMap };

#[derive( Deserialize )]
struct SearchResult { 
    albums : Albums,
    artists : Artists,
    most_popular : MostPopular,
    playlists : Playlists,
    stories : Stories,
    tracks : Tracks,
    query : String,
}
#[derive( Deserialize )]
struct Albums {
    items : HashMap<i32, ItemInfo>
}
#[derive( Deserialize )]
struct Artists {
    items : HashMap<i32, ItemInfo>
}
#[derive( Deserialize )]
struct MostPopular {

}
#[derive( Deserialize )]
struct Playlists { 

}
#[derive( Deserialize )]
struct Stories {

}
#[derive( Deserialize )]
struct Tracks {

}
#[derive ( Deserialize )]
struct ItemInfo {
    id : String,
    release_date_original : String, 
    title : String,
    url : String,
    duration : i32,
    maxium_bit_depth : i32, 
    maximum_channel_count : i32,
    maximum_sampling_rate : f32,
    media_count : i32,
    track_count : i32,
    qobuz_id : i64,
    //artists : Vec<dyn std::any::Any> //specific artist json struct, placeholder any for now

}