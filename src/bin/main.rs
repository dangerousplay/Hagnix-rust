extern crate protobuf;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;
extern crate protoc_rust_grpc;

use grpc::ClientStub;
use grpc::Client;
use grpc::ClientStubExt;
use std::env;
use gRPC::rotmg_grpc::GameClient;
use gRPC::rotmg::EmailRequest;
use gRPC::rotmg_grpc::Game;
use gRPC::rotmg::Empty;

fn main(){
    env_logger::init();

    let name = env::args()
        .filter(|a| a != "--tls")
        .nth(1)
        .map(|s| s.to_owned())
        .unwrap_or_else(|| "world".to_owned());

    let port = 50051;

    let client_conf = Default::default();

    let client = GameClient::new_plain("::1", port, client_conf).unwrap();

    let mut req = EmailRequest::new();

    req.set_email("davificanhahenrique@hotmail.com".to_string());

    let resp = client.authorize_player(grpc::RequestOptions::new(), req);

    let resp2 = client.server_info(grpc::RequestOptions::new(), Empty::new());

    let te = resp2.drop_metadata().wait();

    let players = te.unwrap().players;

    println!("{:?}", resp.wait());
}