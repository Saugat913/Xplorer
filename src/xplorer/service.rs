use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServiceType {
    SERVICE_TYPE_UDP,
    SERVICE_TYPE_TCP,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    service_name: String,
    service_address: String,
    service_port: u32,
    service_type: ServiceType,
}

impl Service {
    pub fn new(
        service_name: String,
        service_address: String,
        service_port: u32,
        service_type: ServiceType,
    ) -> Service {
        Service {
            service_name: service_name,
            service_address: service_address,
            service_port: service_port,
            service_type: service_type,
        }
    }
}
