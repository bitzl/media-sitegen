
use std::fs::read_dir;
use std::path::Path;
use clap::{App, Arg};
use mime_guess::guess_mime_type;

use serde::Serialize;
use tera::{Context, Tera};


#[derive(Serialize)]
enum MediaGroup {
    Audio,
    Video,
    Unknown
}


#[derive(Serialize)]
pub struct MediaFile {
    // the raw filename including the extension, as we assume that
    // all files are in the same directory as the generated HTML
    filename: String,
    // the (guessed) mime-type, needed for the viewers
    mime_type: String,
    // an arbitrary comment. Right now always empty, will be filled when
    // we add optional project files
    comment: String,
    // the overall media group - audio, video or unknown
    media: MediaGroup,
}

fn main() {
    let matches = App::new("Static site generator to present media files")
        .version("0.0.1")
        .author("Marcus Bitzl")
        .about("Creates a static website to quickly present your static media files")
        .arg(
            Arg::with_name("SOURCE")
                .required(true)
                .help("the folder with the media files")
                .index(1),
        )
        .arg(
            Arg::with_name("target_dir")
                .long("target_dir")
                .short("t")
                .value_name("TARGET")
                .required(false)
                .help("the folder to generate html to"),
        )
        .arg(
            Arg::with_name("copy_media_files")
                .long("copy-media-files")
                .short("copy")
                .requires("target_dir")
                .help("copies media files to target dir"),
        )
        .get_matches();

    let mut media_files = Vec::new();
    gather_files(matches.value_of("SOURCE").unwrap(), &mut media_files);

    let mut tera = Tera::default();
    tera.add_raw_template("index.html", include_str!("templates/index.html")).unwrap();
    
    let mut context = Context::new();
    context.insert("media_files", &media_files);
    context.insert("title", "TODO: Happy Title");

    match tera.render("index.html", &context) {
        Ok(html) => println!("{}", html),
        Err(err) => println!("{}", err),
    }
}


// Get all media files in source_dir
fn gather_files(source_dir: &str, media_files: &mut Vec<MediaFile>) {
    let source_path = Path::new(source_dir);
    for entry in read_dir(source_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let filename = match path.file_name() {
            Some(name) => name.to_str().unwrap().to_string(),
            None => panic!("There is a file we cannot get a name for!"),
        };
        let mime_type = guess_mime_type(path).to_string();
        let comment = String::from("");
        let media = determine_media(&mime_type);
        let media_file = MediaFile {
            filename,
            mime_type,
            comment,
            media,
        };
        media_files.push(media_file);
    }
}

// Assign media files to a group to choose a viewer
fn determine_media(mime_type: &String) -> MediaGroup {
    if mime_type.contains("audio") {
        MediaGroup::Audio
    } else if mime_type.contains("video") {
        MediaGroup::Video
    } else {
       MediaGroup::Unknown
    }
}