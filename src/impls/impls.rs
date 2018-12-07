use grpc::RequestOptions;
use grpc::SingleResponse;
use crate::impls::proto::rotmg_grpc::Game;
use crate::impls::proto::rotmg::KickRequest;
use crate::impls::proto::rotmg::Empty;
use crate::impls::proto::rotmg::ListPlayersResponse;
use crate::impls::proto::rotmg::EmailRequest;
use crate::impls::proto::rotmg::Player;
use crate::impls::proto::rotmg::CreatePlayerRequest;
use crate::impls::proto::rotmg::Server;

pub struct GameServerImpl;

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