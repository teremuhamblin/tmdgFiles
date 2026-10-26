#[derive(Debug)]
pub struct Mission {
    pub name: String,
    pub files: Vec<String>,
}

impl Mission {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            files: Vec::new(),
        }
    }
}
