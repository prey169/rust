use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::path::Path;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let json_data = fs::read_to_string("/home/frossi/.cache/ytermusic/downloads/JILfwu5AWIQ.json")?;
//     let data: serde_json::Value = serde_json::from_str(&json_data)?;

//     if let Some(serde_json::Value::String(title)) = data.get("title") {
//         //fs::rename("old_file.txt", &format!("{}.txt", title))?;
//         println!("{}.txt", title);
//     } else {
//         println!("No 'title' field in JSON or it is not a string.");
//     }

//     Ok(())
// }

#[derive(Serialize, Deserialize, Debug)]
struct YtMusicJson {
    title: String,
    author: String,
    album: String,
    video_id: String,
    duration: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let entries = fs::read_dir("/home/frossi/.cache/ytermusic/downloads/")?;

    for entry in entries {
        let entry = entry?;
        let entry_name = entry.file_name().into_string().unwrap();
        if entry_name.ends_with(".json") {
            println!("{}", entry_name);
            let json_data = fs::read_to_string(entry.path().into_os_string())?;
            let data: YtMusicJson = serde_json::from_str(&json_data)?;
            let author_path = format!("/home/frossi/Music/{}", data.author);
            if !Path::new(&author_path).exists() {
                let _ = fs::create_dir(&author_path);
            }
            let album_path = if !data.album.is_empty() {
                format!("/home/frossi/Music/{}/{}", data.author, data.album)
            } else {
                format!("/home/frossi/Music/{}/missing_album", data.author)
            };
            if !Path::new(&album_path).exists() {
                let _ = fs::create_dir(&album_path);
            }
            let new_song_path = format!(
                "/home/frossi/Music/{}/{}/{}.mp4",
                data.author, data.album, data.title
            );
            let current_song_path = format!(
                "/home/frossi/.cache/ytermusic/downloads/{}.mp4",
                entry_name.split('.').take(1).collect::<Vec<_>>()[0]
            );

            if !Path::new(&new_song_path).exists() && Path::new(&current_song_path).exists() {
                let _ = fs::copy(current_song_path, new_song_path);
                println!("{:#?}.txt", data);
            }
        }
    }

    //fs::rename("old_file.txt", &format!("{}.txt", data.title))?;

    Ok(())
}
