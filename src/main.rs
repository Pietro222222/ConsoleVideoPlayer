mod util;
use std::env;

use crate::util::{convert, generate_video_hash};
mod video;

#[tokio::main]
async fn main() {
    if !util::viu_installed() || !util::ffmpeg_installed() {
        panic!("Viu Or Ffmpeg is not installed!!");
    }

    let mut args = env::args().map(|it| it.to_string()).collect::<Vec<_>>();

    if args.len() < 2 {
        println!("Please provide a file");
        return;
    }

    let path = std::path::Path::new(&args[1]);

    if path.is_dir() || !args[1].ends_with(".mp4") {
        println!("Please provide a valid .mp4 file");
        return;
    }
    let hash = util::generate_video_hash(args[1].clone());
    util::create_dirs();
    drop(path);
    let video_in_cache = format!("{}/{}", util::get_cache_dir(), hash);
    let video_in_cache_path = std::path::Path::new(&video_in_cache);

    if video_in_cache_path.exists() {
        util::play_video(
            format!("{}/video.gif", video_in_cache),
            format!("{}/video.mp3", video_in_cache),
        )
        .await;
    } else {
        util::create_video_cache_dir(hash.clone());
        convert(args[1].clone(), hash);
        util::play_video(
            format!("{}/video.gif", video_in_cache),
            format!("{}/video.mp3", video_in_cache),
        )
        .await;
    }

    util::create_dirs();
}
