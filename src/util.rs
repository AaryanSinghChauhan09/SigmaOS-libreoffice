//! libreoffice model — office suite.

#[derive(Debug, Clone)]
pub struct Document {
    pub name: String,
    pub type_: String,
    pub path: String,
    pub modified: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub documents: Vec<Document>,
}

impl Default for Model {
    fn default() -> Self { Self { documents: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn open(&mut self, doc: Document) { self.documents.push(doc); }

    pub fn save(&mut self, name: &str) -> bool {
        if let Some(d) = self.documents.iter_mut().find(|d| d.name == name) {
            d.modified = false; true
        } else { false }
    }

    pub fn list(&self) -> &[Document] { &self.documents }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn open_save() {
        let mut l = Model::new();
        l.open(Document { name: "doc1".into(), type_: "writer".into(), path: "/doc1.odt".into(), modified: false });
        assert!(l.save("doc1"));
    }
}
