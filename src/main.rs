use dotenv::dotenv;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;
use std::process::Command;
fn main() {
    let mut user_input = String::new();
    println!("\x1b[32m Do you have wallpaper Path? type y or Y for yes\x1b[0m ");
    match io::stdin().read_line(&mut user_input) {
        Ok(n) => {
            println!("{n} bytes read");
        }
        Err(error) => println!("error: {error}"),
    }
    let wallpaper_dir = if user_input.trim() == "y" || user_input.trim() == "Y" {
        println!("\x1b[32m Enter wallpaper Path : \x1b[0m");
        let mut wallpaper_path_from_user = String::new();
        match io::stdin().read_line(&mut wallpaper_path_from_user) {
            Ok(n) => {
                println!("{n} bytes read");
            }
            Err(error) => println!("error: {error}"),
        }
        wallpaper_path_from_user
    } else {
        dotenv().ok();
        std::env::var("FOLDER_PATH").expect("folder path  must be set.")
    };
    // command
    // println!("PAth {}", wallpaper_dir);
    let output = Command::new("xrandr").output().expect("Failed to execute xrandr command");
    let output_str = String::from_utf8_lossy(&output.stdout);
    let active_line = output_str
        .lines()
        .find(|line| line.contains(" connected") && line.contains(" primary"))
        .unwrap_or("");
    let resolution = if let Some(resolution) = active_line.split_whitespace().nth(3) {
        let resolution_parts: Vec<&str> = resolution.split('x').collect();
        if resolution_parts.len() == 2 {
            let width = resolution_parts[0].parse::<u32>().unwrap_or(0);
            let height = resolution_parts[1].parse::<u32>().unwrap_or(0);
            if width > height {
                (resolution, "horizontal")
            } else {
                (resolution, "vertical")
            }
        } else {
            ("", "")
        }
    } else {
        ("", "")
    };
    let is_vertical = &resolution.0[0..4] == "1920";
    let mut wallpapers = std::fs
        ::read_dir(wallpaper_dir)
        .expect("Failed to read wallpaper directory")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect::<Vec<_>>();
    let mut rng = thread_rng();
    wallpapers.shuffle(&mut rng);
println!("Array {:? }",wallpapers);
    let wallpaper_path = wallpapers
        .iter()
        .find(|path| {
            let file_name = path.file_name().unwrap().to_string_lossy();
            if is_vertical {
                file_name.contains("hori")
            } else {
                file_name.contains("ver")
            }
        })
        .unwrap_or_else(|| wallpapers.choose(&mut rng).unwrap());
    let gsettings_command = format!(
        "gsettings set org.gnome.desktop.background picture-uri \"file://{}\"",
        wallpaper_path.display()
    );
    Command::new("sh")
        .arg("-c")
        .arg(gsettings_command)
        .output()
        .expect("Failed to execute gsettings command");
}
