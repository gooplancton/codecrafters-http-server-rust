use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum HttpEncodingScheme {
    Gzip,
    None,
}

#[derive(Default, Debug)]
pub struct HttpHeaders(HashMap<String, String>);

impl HttpHeaders {
    pub fn insert(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) {
        let key = key.as_ref().to_owned().to_lowercase();

        self.0.insert(key, value.as_ref().to_owned());
    }

    pub fn get(&mut self, key: impl AsRef<str>) -> Option<&String> {
        let key = key.as_ref().to_owned().to_lowercase();

        self.0.get(&key)
    }

    pub fn remove(&mut self, key: impl AsRef<str>) -> Option<String> {
        let key = key.as_ref().to_owned().to_lowercase();

        self.0.remove(&key)
    }
}

