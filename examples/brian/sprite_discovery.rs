use std::path::Path;

#[derive(Debug)]
pub struct AnimatedSprite {
    pub file_path: String,
    pub tile_name: String,
    pub animation_name: String,
    pub frame_number: u8,
}

pub fn read_animated_tiles<P: AsRef<Path>>(path: P) -> impl Iterator<Item = AnimatedSprite> {
    path.as_ref().read_dir().unwrap().filter_map(|entry| {
        let entry = entry.unwrap();

        // Ensure it's a file with the a png extension. If not, skip it.
        if entry.file_type().unwrap().is_file() && entry.path().extension().unwrap() == "png" {
            let file_name = entry.path().file_stem().unwrap().to_owned();

            let parts: Vec<&str> = file_name.to_str().unwrap().split('_').collect();
            let num_parts = parts.len();

            // Ensure it's an animation tile. If not, skip it.
            if num_parts > 3 && parts[num_parts - 2] == "anim" {
                let frame_number_fragment = parts[num_parts - 1];

                // The last part of a animation tile name must start with 'f'. If not, skip it.
                if frame_number_fragment.chars().next().unwrap() == 'f' {
                    // The tile number must parse to u8. If not, skip it.
                    if let Ok(frame_number) = frame_number_fragment[1..].parse::<u8>() {
                        let tile_name = parts[0..num_parts - 3].join("_");
                        let animation_name = parts[num_parts - 3];

                        Some(AnimatedSprite {
                            animation_name: animation_name.to_owned(),
                            tile_name: tile_name.to_owned(),
                            file_path: entry.path().to_str().unwrap().to_owned(),
                            frame_number,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    })
}
