//! SigmaOS-libreoffice: LibreOffice suite model.

use std::collections::BTreeMap;
use std::fmt;

/// Primary domain entity for LibreOffice suite.
pub struct LibreofficeDocument {
    pub id: String,
    pub name: String,
    pub mime_type: String,
    pub enabled: bool,
}

/// Secondary domain entity for LibreOffice suite.
pub struct LibreofficePlugin {
    pub id: String,
    pub plugin_id: String,
    pub description: String,
}

/// Errors for the LibreOffice suite domain.
#[derive(Debug, PartialEq)]
pub enum LibreofficeError {
    EmptyName,
    Duplicate(String),
    NotFound(String),
}

impl fmt::Display for LibreofficeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "name cannot be empty"),
            Self::Duplicate(id) => write!(f, "duplicate id: {id}"),
            Self::NotFound(id) => write!(f, "not found: {id}"),
        }
    }
}

impl std::error::Error for LibreofficeError {}

/// Registry and manager for LibreOffice suite.
pub struct LibreofficeManager {
    pub name: String,
    items: BTreeMap<String, LibreofficeDocument>,
    configs: BTreeMap<String, LibreofficePlugin>,
}

impl LibreofficeManager {
    /// Create a new manager instance.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: BTreeMap::new(),
            configs: BTreeMap::new(),
        }
    }

    /// Add a primary item with fail-fast validation.
    pub fn add_item(&mut self, item: LibreofficeDocument) -> Result<(), LibreofficeError> {
        if item.name.is_empty() {
            return Err(LibreofficeError::EmptyName);
        }
        if self.items.contains_key(&item.id) {
            return Err(LibreofficeError::Duplicate(item.id.clone()));
        }
        self.items.insert(item.id.clone(), item);
        Ok(())
    }

    /// Retrieve a primary item by id.
    pub fn get_item(&self, id: &str) -> Option<&LibreofficeDocument> {
        self.items.get(id)
    }

    /// Count of primary items.
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Add a secondary configuration entry.
    pub fn add_config(&mut self, cfg: LibreofficePlugin) {
        self.configs.insert(cfg.id.clone(), cfg);
    }

    /// Count of configuration entries.
    pub fn config_count(&self) -> usize {
        self.configs.len()
    }

    /// Enable or disable an item.
    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> Result<(), LibreofficeError> {
        self.items
            .get_mut(id)
            .map(|item| item.enabled = enabled)
            .ok_or_else(|| LibreofficeError::NotFound(id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_LibreofficeDocument(id: &str, name: &str) -> LibreofficeDocument {
        LibreofficeDocument {
            id: id.to_string(),
            name: name.to_string(),
            mime_type: "libreoffice-default".to_string(),
            enabled: true,
        }
    }

    fn make_LibreofficePlugin(id: &str, val: &str) -> LibreofficePlugin {
        LibreofficePlugin {
            id: id.to_string(),
            plugin_id: val.to_string(),
            description: format!("{} config", id),
        }
    }

    #[test]
    fn test_create_aaa() {
        let mgr = LibreofficeManager::new("test");
        assert_eq!(mgr.name, "test");
        assert_eq!(mgr.count(), 0);
        assert_eq!(mgr.config_count(), 0);
    }

    #[test]
    fn test_add_item_aaa() {
        let mut mgr = LibreofficeManager::new("test");
        let item = make_LibreofficeDocument("item1", "First Item");
        assert!(mgr.add_item(item).is_ok());
        assert_eq!(mgr.count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut mgr = LibreofficeManager::new("test");
        let item = make_LibreofficeDocument("item1", "");
        assert_eq!(mgr.add_item(item), Err(LibreofficeError::EmptyName));
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut mgr = LibreofficeManager::new("test");
        let item1 = make_LibreofficeDocument("item1", "First");
        let item2 = make_LibreofficeDocument("item1", "Duplicate");
        mgr.add_item(item1).unwrap();
        assert_eq!(mgr.add_item(item2), Err(LibreofficeError::Duplicate("item1".to_string())));
    }

    #[test]
    fn test_config_addition_aaa() {
        let mut mgr = LibreofficeManager::new("test");
        let cfg = make_LibreofficePlugin("cfg1", "value1");
        mgr.add_config(cfg);
        assert_eq!(mgr.config_count(), 1);
    }

    #[test]
    fn test_get_item_aaa() {
        let mut mgr = LibreofficeManager::new("test");
        let item = make_LibreofficeDocument("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.get_item("item1").is_some());
        assert!(mgr.get_item("none").is_none());
    }

    #[test]
    fn test_set_enabled_aaa() {
        let mut mgr = LibreofficeManager::new("test");
        let item = make_LibreofficeDocument("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.set_enabled("item1", false).is_ok());
        assert!(!mgr.get_item("item1").unwrap().enabled);
        assert_eq!(mgr.set_enabled("none", true), Err(LibreofficeError::NotFound("none".to_string())));
    }
}
