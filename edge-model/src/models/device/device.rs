use std::collections::HashMap;

struct Device {
    id: i32,
    name: String,
    address: Addressable,
    namespace: String,
    resources: Vec(DeviceResource),
    properties: HashMap<String, String>,
}

struct Addressable {}

impl Device {
    fn new(id: i32) -> Self {
        Device {
            id,
            name,
            address: Addressable {},
            namespace,
            resources,
            properties: HashMap::new(),
        }
    }

    pub fn add_device_properties(&mut self, field_name: String, field_value: String) -> Option<String> {
        self.properties.insert(field_name, field_value)
    }
}
