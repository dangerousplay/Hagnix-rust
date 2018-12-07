use std::env;
use hagnix::rotmg_grpc::GameClient;
use hagnix::rotmg::EmailRequest;
use hagnix::rotmg_grpc::Game;
use hagnix::rotmg::Empty;

fn mai(){
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