use std::collections::HashMap;

struct Device {
    id: i32,
    name: String,
    addr: Addressable,
    namespace: String,
    resources: Vec(DeviceResource),
    properties: HashMap,
}

struct Addressable {}

impl Device {
    fn new(id: i32) -> Self {
        Device {
            id,
            name: (),
            addr: Addressable {},
            namespace: (),
            resources: (),
            properties: HashMap::new(),
        }
    }

    pub fn add_device_properties(&mut self, field_name: String, field_value: String) -> bool {
        self.properties.insert(field_name, field_value)
    }
}
