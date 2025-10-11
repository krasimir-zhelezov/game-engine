pub struct Tag {
    pub name: String,
}

impl Tag {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}