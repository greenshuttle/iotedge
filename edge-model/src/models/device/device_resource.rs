enum ResourceScheme {
    Property,
    Telemetry,
    Command,
}

struct DeviceResource {
    id: i32,
    name: String,
    description: String,
    resource_scheme: ResourceScheme,
}