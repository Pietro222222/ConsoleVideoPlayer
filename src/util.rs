// some code here has been stolen from https://github.com/yxqsnz/Console-vid-player/blob/main/src/util.rs
use crate::video::to_gif::convert_to_gif;
use crate::video::to_sound::convert_to_sound;
use crate::video::*;
use core::time;
use sha1;
use std::io::{stdout, BufRead, Read};
use std::io::{BufReader, Write};
use std::process::Command;
use std::thread::spawn;
use std::{env, fs, path::Path};

pub fn viu_installed() -> bool {
    let x = Command::new("viu").output();
    x.is_ok()
}

pub fn ffmpeg_installed() -> bool {
    let x = Command::new("ffmpeg").output();
    x.is_ok()
}

pub fn create_dirs() {
    if let Ok(homedir) = env::var("HOME") {
        let path = format!("{}/.console_player", &homedir);
        let console_player = Path::new(&path);

        if !console_player.exists() || console_player.is_dir() {
            if let Err(err) = fs::create_dir_all(format!("{}/.console_player", homedir.clone())) {
                panic!(
                    "Could not create console_player directory: {}",
                    err.to_string()
                );
            }
            fs::create_dir_all(format!("{}/.console_player/cache", homedir)).unwrap();
        }
    } else {
        panic!("Could not use env var HOME");
    }
}

pub fn get_cache_dir() -> String {
    if let Ok(homedir) = env::var("HOME") {
        return format!("{}/.console_player/cache", homedir);
    } else {
        panic!("Could not get cache dir");
    }
}

pub fn generate_video_hash(path: String) -> String {
    let mut file = fs::File::open(path).unwrap();
    let mut buffer = vec![];
    file.read_to_end(&mut buffer).unwrap();
    let mut sh = sha1::Sha1::new();
    sh.update(&buffer);
    sh.digest().to_string()
}

pub fn create_video_cache_dir(video_hash: String) {
    if let Err(err) = fs::create_dir_all(format!("{}/{}", get_cache_dir(), video_hash)) {
        panic!("Error while creating cache dir");
    }
}

pub async fn play_video(video_file: String, sound_file: String) {
    //play video
    std::thread::spawn({
        move || {
            std::thread::sleep(time::Duration::from_millis(500));
            play::play(sound_file);
        }
    });
    let _ = Command::new("viu")
        .args(&[video_file, "--once".to_string()])
        .spawn()
        .unwrap()
        .wait();
    //std::thread::spawn({ move || play::play(sound_file) });
    let _ = stdout().flush();
    //std::thread::spawn({ move || play::play(sound_file) });
}
pub fn convert(video_file: String, video_hash: String) {
    let path = format!("{}/{}", get_cache_dir(), video_hash);
    println!("{} {}", &path, video_file);
    let sound_path = format!("{}/video.mp3", &path);
    let gif_path = format!("{}/video.gif", &path);
    convert_to_sound(video_file.clone(), sound_path);
    convert_to_gif(video_file, gif_path);
}
