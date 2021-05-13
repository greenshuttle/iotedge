use std::collections::HashMap;
use crate::device::model::device_profile;

struct Device {
    id: i32,
    name: String,
    address: Addressable,
    namespace: String,
    profile: device_profile::DeviceProfile,
    properties: HashMap<String, String>,
}

struct Addressable {}

impl Device {
    fn new(id: i32, profile : device_profile::DeviceProfile) -> Self {
        Device {
            id,
            name:  "Pascal".to_string(),
            address: Addressable {},
            namespace: "Namespace".to_string(),
            profile,
            properties: HashMap::new(),
        }
    }

    pub fn add_device_properties(&mut self, field_name: String, field_value: String) -> Option<String> {
        self.properties.insert(field_name, field_value)
    }
}
