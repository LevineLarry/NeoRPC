use std::fs;
use yaml_rust::YamlLoader;

#[derive(Debug, Clone)]
pub struct EventParameter {
    pub name: String,
    pub type_: String,
    pub description: String
}

#[derive(Debug, Clone)]
pub struct Event {
    pub name: String,
    pub description: String,
    pub parameters: Vec<EventParameter>,
    pub public: bool
}

#[derive(Debug, Clone)]
pub struct ProcParameter {
    pub name: String,
    pub type_: String,
    pub description: String,
    pub required: bool,
}

#[derive(Debug, Clone)]
pub struct Procedure {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ProcParameter>,
    pub public: bool,
    pub emits: Vec<String>
}

#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub description: String,
    pub events: Vec<Event>,
    pub procedures: Vec<Procedure>
}

#[derive(Debug, Clone)]
pub struct Pod {
    pub name: String,
    pub description: String,
    pub services: Vec<Service>
}

pub fn parse(declaration_path: String) -> Pod {
    let contents = fs::read_to_string(declaration_path).expect("Unable to find pod declaration file");
    let yaml = YamlLoader::load_from_str(&contents).unwrap();
    let name = yaml[0]["name"].as_str().unwrap().to_string();
    let description = yaml[0]["description"].as_str().unwrap().to_string();

    let services = yaml[0]["services"].as_vec().unwrap();
    let parsed_services = services.iter().map(|service| {
        let events = service["events"].as_vec().unwrap();
        let parsed_events = events.iter().map(|event| {
            parse_event(event)
        }).collect::<Vec<Event>>();

        let procs = service["procs"].as_vec().unwrap();
        let parsed_procs = procs.iter().map(|proc| {
            parse_procedure(proc, &parsed_events)
        }).collect::<Vec<Procedure>>();

        Service {
            name: service["name"].as_str().unwrap().to_string(),
            description: service["description"].as_str().unwrap().to_string(),
            events: parsed_events,
            procedures: parsed_procs
        }
    }).collect::<Vec<Service>>();

    Pod {
        name,
        description,
        services: parsed_services
    }
}

fn parse_event(event: &yaml_rust::Yaml) -> Event {
    let name = event["name"].as_str().unwrap();
    let description = event["description"].as_str().unwrap();
    let public = event["public"].as_bool().unwrap_or(false);
    let params = event["params"].as_vec().unwrap();

    let mut parsed_params: Vec<EventParameter> = Vec::new();
    params.iter().for_each(|param| {
        let name = param["name"].as_str().unwrap();
        let type_ = param["type"].as_str().unwrap();
        let description = param["description"].as_str().unwrap();

        parsed_params.push(EventParameter {
            name: name.to_string(), 
            type_: type_.to_string(), 
            description: description.to_string()
        });
    });

    Event {
        name: name.to_string(),
        description: description.to_string(),
        parameters: parsed_params,
        public
    }
}

fn parse_procedure(procedure: &yaml_rust::Yaml, events: &Vec<Event>) -> Procedure {
    let name = procedure["name"].as_str().unwrap();
    let description = procedure["description"].as_str().unwrap();
    let public = procedure["public"].as_bool().unwrap_or(false);
    let params = procedure["params"].as_vec().unwrap();

    let mut parsed_params: Vec<ProcParameter> = Vec::new();
    params.iter().for_each(|param| {
        let name = param["name"].as_str().unwrap();
        let type_ = param["type"].as_str().unwrap();
        let description = param["description"].as_str().unwrap();
        let required = param["required"].as_bool().unwrap_or(false);

        parsed_params.push(ProcParameter {
            name: name.to_string(), 
            type_: type_.to_string(), 
            description: description.to_string(),
            required
        });
    });

    let emitted_events = procedure["emits"].as_vec().unwrap().iter().map(|event| {
        if !events.iter().any(|e| e.name == event.as_str().unwrap().to_string()) {
            panic!("Procedure emits event that does not exist");
        }

        event.as_str().unwrap().to_string()
    }).collect::<Vec<String>>();



    Procedure {
        name: name.to_string(),
        description: description.to_string(),
        parameters: parsed_params,
        public,
        emits: emitted_events
    }
}