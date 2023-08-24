mod neorpc_server;
use neorpc_server::server::{self, UsersService};

fn main() {
    println!("Hello, world!");
}

pub struct TestServer;

impl UsersService for TestServer {
    fn handle_sign_up(username: String, email: String, password: String) {
        todo!()
    }

    fn handle_sign_in(username: String, password: String, attempt: i32) {
        todo!()
    }

    fn emit_account_created(id: i32, email: String) {
        todo!()
    }

    fn emit_email_validated(id: i32, timestamp: i32) {
        todo!()
    }

    fn emit_sign_in_complete(id: i32, timestamp: i32) {
        todo!()
    }
}