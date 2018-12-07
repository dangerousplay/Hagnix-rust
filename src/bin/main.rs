extern crate futures;

extern crate grpc;
extern crate tls_api;
extern crate tls_api_native_tls;

use std::env;
use std::thread;
use grpc::ServerBuilder;

use tls_api::TlsAcceptorBuilder;
use grpc::RequestOptions;
use hagnix::rotmg::KickRequest;
use grpc::SingleResponse;
use hagnix::rotmg::Empty;
use hagnix::rotmg::Player;
use hagnix::rotmg::Server;
use hagnix::rotmg::EmailRequest;
use hagnix::rotmg::CreatePlayerRequest;
use hagnix::rotmg_grpc::Game;
use hagnix::rotmg::ListPlayersResponse;
use tls_api_native_tls::TlsAcceptor;
use hagnix::rotmg_grpc::GameServer;

struct GameServerImpl;

impl Game for GameServerImpl {
    fn kick_player(&self, o: RequestOptions, p: KickRequest) -> SingleResponse<Empty> {
        return SingleResponse::completed(Empty::new());
    }

    fn list_players(&self, o: RequestOptions, p: Empty) -> SingleResponse<ListPlayersResponse> {
        unimplemented!()
    }

    fn get_player(&self, o: RequestOptions, p: EmailRequest) -> SingleResponse<Player> {
        unimplemented!()
    }

    fn ban_player(&self, o: RequestOptions, p: EmailRequest) -> SingleResponse<Empty> {
        unimplemented!()
    }

    fn pardon_player(&self, o: RequestOptions, p: EmailRequest) -> SingleResponse<Empty> {
        unimplemented!()
    }

    fn logged_player(&self, o: RequestOptions, p: EmailRequest) -> SingleResponse<Empty> {
        unimplemented!()
    }

    fn authorize_player(&self, o: RequestOptions, p: EmailRequest) -> SingleResponse<Empty> {
        unimplemented!()
    }

    fn create_player(&self, o: RequestOptions, p: CreatePlayerRequest) -> SingleResponse<Player> {
        unimplemented!()
    }

    fn delete_player(&self, o: RequestOptions, p: EmailRequest) -> SingleResponse<Empty> {
        unimplemented!()
    }

    fn change_player(&self, o: RequestOptions, p: Player) -> SingleResponse<Empty> {
        unimplemented!()
    }

    fn server_info(&self, o: RequestOptions, p: Empty) -> SingleResponse<Server> {
        unimplemented!()
    }
}

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