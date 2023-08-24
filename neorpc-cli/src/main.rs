use generator::Generator;

/**
 * Users can define their services and the procedures within their services using yaml
 * A group of services is called a pod
 * Users can deploy pods to the cloud and list them as public or private
 * Users can discover public pods and use their services
 * Users can discover private pods as long as they have an api key
 * Code generation is done for each service and procedure, both when developing your own and using somebody elses
 * Procedures can emit events which users can then subscribe to
 * Generate graph/node diagram of how pods interact
 * Integrate cli with chatgpt to allow it to answer questions about services
 * Built on top of tcp
 * SAAS?
 */

use parser::parse;
use std::env;

mod generator;
mod parser;
mod resolver_server;
pub mod helpers;

//neorpc generate -pod ./neorpc_pods/test-pod.yaml -lang rust -out ./neorpc-server
fn main() {
    let args: Vec<String> = env::args().collect();
    let invocation_dir = env::current_dir().unwrap().to_str().unwrap().to_string();

    println!("{:?}", args);

    if args.len() < 5 {
        panic!("Invalid invocation");
    }

    if args[1] == "generate" {
        if args[2] == "-pod" {
            if args.len() < 4 {
                panic!("Please provide a path to a pod declaration file");
            }

            let pod_location = invocation_dir.clone() + "/" + &args[3].clone();
            println!("Searching for pod file in {}", pod_location.clone());

            if args[4] == "-lang" {
                if args.len() < 6 {
                    panic!("Please provide a language to generate code for");
                }

                let lang = args[5].clone();

                if lang == "rust" {
                    if args.len() == 8 {
                        if args[6] == "-out" {
                            if args.len() < 8 {
                                panic!("Please provide a path to output the generated code to");
                            }
    
                            let out_dir = args[7].clone();
    
                            let pod = parse(pod_location); //Path is relative to dir that cargo run is called from
                            Generator::generate_server_trait(&pod, out_dir.clone());
                            Generator::generate_client_trait(&pod, out_dir.clone());
                        }
                    } else {
                        println!("Outputting to {}", invocation_dir.clone());

                        let pod = parse(pod_location); //Path is relative to dir that cargo run is called from
                        Generator::generate_server_trait(&pod, invocation_dir.clone());
                        Generator::generate_client_trait(&pod, invocation_dir.clone());
                    }
                }

                if args.len() == 7 {
                    panic!("Inavlid command");
                }
            }
        }
    } else {
        panic!("Invalid command");
    }
}