use std::collections::HashMap;
use crate::models::device::device_resource;

struct Device {
    id: i32,
    name: String,
    address: Addressable,
    namespace: String,
    resource: device_resource::DeviceResource,
    properties: HashMap<String, String>,
}

struct Addressable {}

impl Device {
    fn new(id: i32, resource : device_resource::DeviceResource) -> Self {
        Device {
            id,
            name:  "Pascal".to_string(),
            address: Addressable {},
            namespace: "Namespace".to_string(),
            resource,
            properties: HashMap::new(),
        }
    }

    pub fn add_device_properties(&mut self, field_name: String, field_value: String) -> Option<String> {
        self.properties.insert(field_name, field_value)
    }
}
