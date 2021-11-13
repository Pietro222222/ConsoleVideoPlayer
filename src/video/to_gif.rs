use std::process::Command;

pub fn convert_to_gif(input: String, output: String) {
    let _ = Command::new("ffmpeg")
        .args(&["-i", input.as_str(), output.as_str()])
        .output()
        .unwrap();
}
