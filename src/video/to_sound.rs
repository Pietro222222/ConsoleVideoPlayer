use std::process::Command;

pub fn convert_to_sound(input: String, output: String) {
    println!("{} {}", input, output);

    let _ = Command::new("ffmpeg")
        .args(&["-i", input.as_str(), output.as_str()])
        .output()
        .unwrap();
}
