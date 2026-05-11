use std::{fs, env, process::Command};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let home = env::var("HOME").unwrap();
    let dir = format!("{}/Perso/auto-wallpaper-42/wallpapers", home);

    let extensions = ["jpg", "jpeg", "png", "webp", "bmp"];
    let mut images: Vec<_> = fs::read_dir(&dir)
        .expect("Directory not found")
        .filter_map(|e| {
            let path = e.ok()?.path();
            let ext = path.extension()?.to_str()?.to_lowercase();
            extensions.contains(&ext.as_str()).then(|| path.to_string_lossy().into_owned())
        })
        .collect();

    if images.is_empty() {
        eprintln!("No images found");
        std::process::exit(1);
    }

    images.sort();
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as usize;
    let chosen = &images[seed % images.len()];

    for key in ["picture-uri", "picture-uri-dark"] {
        Command::new("gsettings")
            .args(["set", "org.gnome.desktop.background", key, &format!("file://{}", chosen)])
            .status().ok();
    }
}
