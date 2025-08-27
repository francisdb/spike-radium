use log::info;
use std::io::Result;
use std::io::Read;
use std::{path};
use std::fs::File;

use byteorder::{LittleEndian, ReadBytesExt};

fn main() -> Result<()> {
    // Set default logging level to info
    // Initialize the logger with color support and debug level
    env_logger::Builder::new()
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        .filter_level(log::LevelFilter::Debug)
        .init();

    info!("Starting the scan...");

    // do a recursive search of *.radium files in ./lcd
    let radium_files = find_radium_files(path::Path::new("./lcd"))?;

    // for now we filter out files where the path contains "5b2d86be0f5ed1a1e3d4a527fc8a0aa8113d285d"
    let radium_files = radium_files.into_iter().filter(|p| {
        let s = p.to_str().unwrap_or("");
        //s.contains("5b2d86be0f5ed1a1e3d4a527fc8a0aa8113d285d")
        s.contains("60ed7e5036b8ce09d35a3e101ea6fc1380b37d97")
    }).collect::<Vec<_>>();

    // for each file try t parse it by reading bytes
    for file in radium_files {
        parse_radium_file(&file)?;
    }

    info!("Done parsing radium files.");
    Ok(())
}

fn find_radium_files(dir: &path::Path) -> Result<Vec<path::PathBuf>> {
    let mut radium_files = Vec::new();
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                radium_files.extend(find_radium_files(&path)?);
            } else if let Some(ext) = path.extension() {
                if ext == "radium" {
                    radium_files.push(path);
                }
            }
        }
    }
    Ok(radium_files)
}

fn parse_radium_file(file_path: &path::Path) -> Result<()> {
    let file_size = std::fs::metadata(file_path)
        .expect("Failed to get file metadata")
        .len();
    // open the file
    info!("Parsing file: {:?} ({})", file_path, file_size);

    let mut file = std::fs::File::open(file_path).expect("Failed to open file");

    // first byte always 0x01

    let mut buffer = [0; 1];
    file.read_exact(&mut buffer).expect("Failed to read file");
    assert_eq!(buffer, [0x01]);

    // second 8 byte seems to be related to the amount of blocks in the file?

    let mut buffer = [0; 8];
    file.read_exact(&mut buffer).expect("Failed to read file");
    info!("Block count?: {:X?}", buffer);

    // then follow 4 bytes indicating the number of sections in the block?
    let mut buffer = [0; 4];
    file.read_exact(&mut buffer).expect("Failed to read file");
    info!("Section count?: {:X?}", buffer);


    // Block type strings we have seen so far:
    // - Font
    // - Video
    // - VideoSurface
    // - Text
    // - Sprite
    // - Shape



    loop {
        // read a block
        let _ = read_id_header(&mut file)?;

        // read a section type
        let section_type = read_string(&mut file).expect("Failed to read string");
        info!("section type: {}", section_type);

        // switch on type
        match section_type.as_str() {
            "Video" => {
                // read the video description
                // read 2 headers, not sure why
                let _ = read_id_header(&mut file)?;
                let _ = read_header(&mut file)?;
                let description = read_string(&mut file).expect("Failed to read string");
                let video_section = VideoSection { description };
                info!("Video section: {:?}", video_section);
                let _ = read_header(&mut file)?;
                // read 9 bytes we currently don't understand
                let mut unknown_buffer = [0; 9];
                file.read_exact(&mut unknown_buffer).expect("Failed to read file");
                info!("Video section unknown 9 bytes: {:X?}", unknown_buffer);

                let video_count = file.read_u64::<LittleEndian>().expect("Failed to read video count");
                info!("Videos section count: {}", video_count);

                for i in 0..video_count {
                    // FIXME header reading is probably wrong inside read_video
                    let video = read_video(&mut file)?;
                    info!("Video section video {}: {:?}", i, video);
                }
            },
            _ => {
                info!("Unknown section type: {}", section_type);
                // is there a way to skip unknown blocks? Do we have a length prefix somewhere? Can we just read until the next known header?
                break;
            }
        }
    }

    Ok(())
}

fn read_id(file: &mut File) -> Result<u16> {
    let id = file.read_u16::<LittleEndian>().expect("Failed to read id");
    Ok(id)
}

fn read_header(file: &mut File) -> Result<[u8; 4]> {
    // before a string we always see 4 bytes
    let mut buffer = [0; 4];
    file.read_exact(&mut buffer).expect("Failed to read header");
    info!("Header 4 bytes: {:X?}", buffer);
    Ok(buffer)
}

fn read_id_header(file: &mut File) -> Result<(u16,[u8; 2])> {
    let id = read_id(file)?;
    // not sure what these 2 bytes are
    let mut remaining = [0; 2];
    file.read_exact(&mut remaining).expect("Failed to read file");
    info!("ID {} {:X?}", id, remaining);
    Ok((id, remaining))
}

#[derive(Debug)]
struct VideoSection {
    description: String,
}

#[derive(Debug)]
struct Video {
    name: String,
    path: String,
}

fn read_video(file: &mut File) -> Result<Video> {
    let name = read_string(file)?;
    let _ = read_id_header(file)?;
    let path = read_string(file)?;
    // unknown 4 bytes
    let mut unknown_buffer = [0; 4];
    file.read_exact(&mut unknown_buffer).expect("Failed to read file");
    info!("Video unknown 4 bytes: {:X?}", unknown_buffer);
    Ok(Video { name, path })
}


/// A string consists of a 8 bytes length prefix followed by the string data
fn read_string(file: &mut File) -> Result<String> {
    let mut length_buffer = [0; 8];
    file.read_exact(&mut length_buffer)?;
    let length = u64::from_le_bytes(length_buffer) as usize;
    if length > 1024 {
        panic!("String length too long: {} ({:?})", length, length_buffer);
    }
    let mut string_buffer = vec![0; length];
    file.read_exact(&mut string_buffer)?;
    let string = String::from_utf8_lossy(&string_buffer).to_string();
    Ok(string)
}
