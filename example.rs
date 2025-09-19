// Example Rust code file for the text file viewer
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HMIComponent {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub properties: HashMap<String, String>,
}

impl HMIComponent {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            enabled: true,
            properties: HashMap::new(),
        }
    }
    
    pub fn set_property(&mut self, key: &str, value: &str) {
        self.properties.insert(key.to_string(), value.to_string());
    }
    
    pub fn get_property(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }
    
    pub fn toggle_enabled(&mut self) {
        self.enabled = !self.enabled;
    }
}

// Example usage
fn main() {
    let mut button = HMIComponent::new("btn_1", "Primary Button");
    button.set_property("color", "blue");
    button.set_property("size", "large");
    
    println!("Component: {:?}", button);
    println!("Color: {:?}", button.get_property("color"));
    
    button.toggle_enabled();
    println!("Enabled: {}", button.enabled);
}
