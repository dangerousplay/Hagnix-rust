extern crate futures;

extern crate grpc;
extern crate tls_api;
extern crate tls_api_native_tls;

use std::env;
use std::thread;
use grpc::ServerBuilder;

use tls_api::TlsAcceptorBuilder;
use tls_api_native_tls::TlsAcceptor;
use crate::impls::proto::rotmg_grpc::GameServer;
use crate::impls::impls::GameServerImpl;

mod impls;

fn main() {

    let port = 50051;

    let mut server: ServerBuilder<TlsAcceptor> = ServerBuilder::new();
    server.http.set_port(port);
    server.add_service(GameServer::new_service_def(GameServerImpl));
    server.http.set_cpu_pool_threads(4);

    let _server = server.build().expect("server");

    loop {
        thread::park();
    }
}