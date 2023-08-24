use crate::parser::Pod;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ResolvablePod {
    pub pod: Pod,
    pub uuid: String,
    pub ip: String,
    pub port: u16
}

pub struct Resolver {
    pub pods: Vec<ResolvablePod>
}

impl Resolver {
    pub fn new() -> Resolver {
        Resolver {
            pods: Vec::new()
        }
    }

    pub fn add_pod(&mut self, pod: Pod, ip: String, port: u16) {
        let uuid = Uuid::new_v4().to_string();
        self.pods.push(ResolvablePod {
            pod,
            uuid,
            ip,
            port
        });
    }

    pub fn resolve(&self, pod_name: String) -> Option<String> {
        for pod in &self.pods {
            if pod.pod.name == pod_name {
                return Some(pod.uuid.clone());
            }
        }

        None
    }

    pub fn get_pod(&self, uuid: String) -> Option<&ResolvablePod> {
        for pod in &self.pods {
            if pod.uuid == uuid {
                return Some(pod);
            }
        }

        None
    }
}