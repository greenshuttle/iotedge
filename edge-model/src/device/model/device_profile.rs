use crate::device::model::device_resource;
use std::collections::HashMap;

pub struct DeviceProfile {
    id: i32,
    name: String,
    namespace: String,
    resource: device_resource::DeviceResource,
    properties: HashMap<String, String>,
}

impl DeviceProfile {
    fn new(id: i32, resource: device_resource::DeviceResource) -> Self {
        DeviceProfile {
            id,
            name: "Pascal".to_string(),
            namespace: "Namespace".to_string(),
            resource,
            properties: HashMap::new(),
        }
    }

    pub fn add_device_profile_properties(
        &mut self,
        field_name: String,
        field_value: String,
    ) -> Option<String> {
        self.properties.insert(field_name, field_value)
    }
}
