#[macro_use] extern crate rocket;

extern crate glob;
extern crate serde_yaml;
extern crate serde;

use std::fs;

use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::futures::TryFutureExt;
use rocket::response::status::NotFound;
use rocket::serde::Deserialize;

use reqwest::Client;

const SONGS_DIRECTORY: &str = "./songs/";

#[derive(FromForm)]
struct SongArguments {
  #[field(name = "songID")]
  song_id: String
}

#[derive(Debug)]
struct SongData {
  id: String,
  name: String,
  artist: String,
  url: String,
  size: String
}

#[derive(Deserialize, Debug)]
struct SongYaml {
  id: String,
  name: String,
  artist: String,
  file: String
}

fn create_directory(directory: &str) {
  if !fs::metadata(directory).is_ok() {
    fs::create_dir(directory).unwrap();
  }
}

fn format_song(song_data: SongData) -> String {
  format!("1~|~{}~|~2~|~{}~|~3~|~0~|~4~|~{}~|~5~|~{:.2}~|~6~|~~|~7~|~~|~8~|~0~|~10~|~{}", 
    song_data.id,
    song_data.name,
    song_data.artist,
    song_data.size.parse::<f64>().unwrap_or(0.0) / (1024. * 1024.),
    song_data.url
  )
}

async fn pass_to_geometry_dash(song_id: i32) -> String {
  let client = Client::new();
  let url = format!("https://www.boomlings.com/database/getGJSongInfo.php?songID={}", song_id);
  let form = reqwest::multipart::Form::new()
    .text("songID", song_id.to_string())
    .text("secret", "Wmfd2893gb7");
  let request = client.post(&url)
    .multipart(form)
    .build()
    .unwrap();

  let response = client.execute(request).await;

  if let Ok(response) = response {
    return response.text().await.unwrap();
  }

  String::new()
}

fn fetch_local_song(song_id: i32) -> Option<SongData> {
  let yml_files = glob::glob(
    &format!("{SONGS_DIRECTORY}*.yaml")
  ).unwrap();

  for entry in yml_files {
    if let Ok(path) = entry {
      let file_data = fs::read_to_string(path).unwrap();
      let song_yaml: SongYaml = serde_yaml::from_str(&file_data).unwrap();

      if song_yaml.id.clone() != song_id.to_string() {
        continue;
      }

      let file_size = fs::metadata(&format!("{SONGS_DIRECTORY}/{}", song_yaml.file)).unwrap().len();
      let song_data = SongData {
        id: format!("-{}", song_yaml.id.clone()),
        name: song_yaml.name,
        artist: song_yaml.artist,
        url: format!("localhost:{}/get_song/{}", 8000, song_yaml.file),
        size: file_size.to_string()
      };

      return Some(song_data);
    }
  }

  None
}

#[post("/", data = "<form>")]
async fn song_data(form: Form<SongArguments>) -> String {
  let song_id = form.song_id.parse::<i32>().unwrap_or(0);

  if song_id < 0 {
    let song_id = -song_id;
    if let Some(song_data) = fetch_local_song(song_id) {
      let data = format_song(song_data);
      println!("{}", data);
      return data;
    }
  } else {
    return pass_to_geometry_dash(song_id).await;
  }

  String::new()
}

#[get("/<song_name>")]
async fn get_local_song(song_name: &str) -> Result<NamedFile, NotFound<String>> {
  let path = format!("{SONGS_DIRECTORY}/{}", song_name);
  NamedFile::open(&path).map_err(|e| NotFound(e.to_string())).await
}

#[launch]
fn rocket() -> _ {
  create_directory(SONGS_DIRECTORY); // Just incase if the directory doesn't exist

  rocket::build()
    .mount("/song_data", routes![song_data])
    .mount("/get_song", routes![get_local_song])
}