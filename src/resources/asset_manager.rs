use std::{collections::HashMap, sync::Arc};

pub struct Texture {
    pub image_data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub struct AssetManager {
    textures: HashMap<String, Arc<Texture>>,
}

impl AssetManager {
    pub fn new() -> Self {
        let mut manager = Self {
            textures: HashMap::new(),
        };
        manager.load_textures();
        manager
    }

    fn load_texture(&mut self, name: &str) -> &Texture {
        let img = image::open("assets/".to_string() + name).expect("Failed to load image file");
        let rgba_image = img.into_rgba8();
        let (width, height) = rgba_image.dimensions();
        let image_data = rgba_image.into_raw();

        if !self.textures.contains_key(name) {
            let texture = Arc::new(Texture {
                image_data,
                width,
                height,
            });
            
            self.textures.insert(name.to_string(), texture);
        }
        self.textures.get(name).unwrap()
    }

    fn load_textures(&mut self) {
        let dir_path = "assets/";

        for entry in std::fs::read_dir(dir_path).expect("Failed to read assets directory") {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "png" || ext == "jpg" || ext == "jpeg" {
                        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                            self.load_texture(file_name);
                        }
                    } else {
                        eprintln!("Unsupported file format: {:?}", path);
                    }
                }
            }
        }
    }

    pub fn get_texture(&self, name: &str) -> Option<Arc<Texture>> {
        self.textures.get(name).cloned()
    }
}