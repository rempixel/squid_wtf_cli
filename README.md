# A cmd line program to install flacs.

# Usage and Compilation: 
*How to use!*

**Installation:** 
- Install `rustc` on your machine
- run `cargo build --release` on the project.
- run the subsequent program executable.
- *Plan to release compiled program in the future!*

**Usage:** 
- When prompted to search, input your result with
	- `{artist name} - {album name}`

# Plan: 
*Some rudimentary research and thinking process!*

- Search album using GET request with: 
	- "get-music?q={query_here}&offset={offset_here}"
	- within resulting page results in raw data file. 
	- can filter through by matching album names.
	- string format to find album id: 
		- For example: Worlds is 
		- ``"title":"Worlds","qobuz_id":18454080,"version":null,"url":"https://www.qobuz.com/fr-fr/album/worlds-porter-robinson/0060253770732","duration":3463,"parental_warning":false,"popularity":0,"tracks_count":12,"genre":{"path":[64],"color":"#5eabc1","name":"Electronic","id":64,"slug":"electro"}
		- We're looking for the id within that url. There's also an id field we can pull from
	- Trouble: Unsure as to how to find track_id.
		- SOLUTION: get-album lists all track ids:
			- `"track_ids":[18454082,18454084,18454085,18454086,18454088,18454090,18454093,18454096,18454099,18454102,18454106,18454109]`
- To convert .flac files to .m4a files using alac compression with 16 bit planar and a max hertz of 44100.
	- `for i in *.flac; do ffmpeg -i "$i" -c:v copy -sample_fmt s16p -ar 44100 -c:a alac "${i%.*}.m4a"; done` 

# Part of a larger personal iPod revival project:

**Parts to Purchase**
*Certain things are broke with the pod*
- [x] [SD Card](https://www.amazon.com/dp/B0B7NS71G2?_encoding=UTF8&th=1)
- [x] [iFlash Converter](https://www.iflash.xyz/store/4th-gen-iflash-converter/)
- [x] [iFlash-ATA1](https://www.iflash.xyz/store/iflash-ata1/)
- Replacement Battery
	- [x] [Infant Optics](https://www.amazon.com/dp/B00EDQ6LZ0) (This option will require some work to re-solder connectors)
	- [ ] Some already pre-soldered ones  I'll find something

Total Cost: ~$65ish dollars? 

**iPod successfully repaired!**

# Projects to Create 

**Recursively Download from Playlist!** 

- [rust-ytdl crate](https://crates.io/crates/rusty_ytdl) (assuming this crate works)
- On launch, guess and check YT rate limit, Go off last known rate limit and test. 
	- If fail - Proxy to switch on HTTP Status Code 429
		- Set new rate limit 
	- If pass proceed
	- Could be swapped to a separate program to constantly ping yt to try rate limit
- [Look into switching Ipv6 on rate limit to extend on proxy idea](https://www.iana.org/assignments/ipv6-unicast-address-assignments/ipv6-unicast-address-assignments.xhtml)
- Go into playlist
	- recursively run through list length
	- grab video object and throw into ytdl
- break on HTTP Status Code 429 or end of list
