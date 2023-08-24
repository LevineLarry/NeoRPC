use std::{fs::{self, File}, io::Write};

use codegen::Scope;

use crate::{parser::{Procedure, Event, Pod}, helpers::capitalize_first};

use self::type_resolver::TypeResolver;

mod type_resolver;

pub struct Generator;

impl Generator {
    fn generate_proc_documentation(proc: &Procedure) -> String {
        format!("{} {}", proc.description, proc.parameters.iter().map(|param| {
            format!("\n@param {} {}", param.name, param.description)
        }).collect::<Vec<String>>().join(""))
    }
    
    fn generate_event_documentation(event: &Event) -> String {
        format!("{} {}", event.description, event.parameters.iter().map(|event| {
            format!("\n@param {} {}", event.name, event.description)
        }).collect::<Vec<String>>().join(""))
    }
    
    pub fn generate_server_trait(pod: &Pod, out_dir: String) {
        let mut scope = Scope::new();
        
        println!("Generating server code at {}", format!("{}/neorpc_server", out_dir));
        fs::create_dir_all(format!("{}/neorpc_server", out_dir)).unwrap();

        let mut mod_file = File::create(format!("{}/neorpc_server/mod.rs", out_dir)).unwrap();
        mod_file.write_all(b"pub mod server;").unwrap();

        let mut file = File::create(format!("{}/neorpc_server/server.rs", out_dir)).unwrap();
    
        pod.services.iter().for_each(|service| {
            let trait_name = format!("{}Service", capitalize_first(service.name.replace("-","_").as_str()).as_str());
            let service_trait = scope.new_trait(trait_name).vis("pub");
    
            service.procedures.iter().for_each(|proc| {
                let proc_func = service_trait.new_fn(format!("handle_{}", proc.name.as_str()));
                proc_func.doc(Self::generate_proc_documentation(proc).as_str());
    
                proc.parameters.iter().for_each(|param| {
                    let _type = TypeResolver::resolve(&param.type_);
                    proc_func.arg(param.name.as_str(), _type);
                });
            });
    
            service.events.iter().for_each(|event| {
                let event_func = service_trait.new_fn(format!("emit_{}", event.name.as_str()));
                event_func.doc(Self::generate_event_documentation(event).as_str());
    
                event.parameters.iter().for_each(|param| {
                    let _type = TypeResolver::resolve(&param.type_);
                    event_func.arg(param.name.as_str(), _type);
                });
            });
        });
    
        file.write_all(scope.to_string().as_bytes()).unwrap();
    }
    
    pub fn generate_client_trait(pod: &Pod, out_dir: String) {
        let mut scope = Scope::new();
        
        fs::create_dir_all(format!("{}/neorpc_client", out_dir)).unwrap();

        let mut mod_file = File::create(format!("{}/neorpc_client/mod.rs", out_dir)).unwrap();
        mod_file.write_all(b"pub mod client;").unwrap();

        let mut file = File::create(format!("{}/neorpc_client/client.rs", out_dir)).unwrap();
    
        pod.services.iter().for_each(|service| {
            let trait_name = format!("{}Service", capitalize_first(service.name.replace("-","_").as_str()).as_str());
            let service_trait = scope.new_trait(trait_name).vis("pub");
    
            service.procedures.iter().for_each(|proc| {
                let proc_func = service_trait.new_fn(format!("exec_{}", proc.name.as_str()));
                proc_func.doc(Self::generate_proc_documentation(proc).as_str());
    
                proc.parameters.iter().for_each(|param| {
                    let _type = TypeResolver::resolve(&param.type_);
                    proc_func.arg(param.name.as_str(), _type);
                });
            });
    
            service.events.iter().for_each(|event| {
                let event_func = service_trait.new_fn(format!("handle_{}", event.name.as_str()));
                event_func.doc(Self::generate_event_documentation(event).as_str());
    
                event.parameters.iter().for_each(|param| {
                    let _type = TypeResolver::resolve(&param.type_);
                    event_func.arg(param.name.as_str(), _type);
                });
            });
        });
    
        file.write_all(scope.to_string().as_bytes()).unwrap();
    }
}
