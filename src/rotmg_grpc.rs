// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

use std::string::ToString;

pub trait Game {
    fn kick_player(&self, o: ::grpc::RequestOptions, p: super::rotmg::KickRequest) -> ::grpc::SingleResponse<super::rotmg::Empty>;

    fn list_players(&self, o: ::grpc::RequestOptions, p: super::rotmg::Empty) -> ::grpc::SingleResponse<super::rotmg::ListPlayersResponse>;

    fn get_player(&self, o: ::grpc::RequestOptions, p: super::rotmg::EmailRequest) -> ::grpc::SingleResponse<super::rotmg::Player>;

    fn ban_player(&self, o: ::grpc::RequestOptions, p: super::rotmg::EmailRequest) -> ::grpc::SingleResponse<super::rotmg::Empty>;

    fn pardon_player(&self, o: ::grpc::RequestOptions, p: super::rotmg::EmailRequest) -> ::grpc::SingleResponse<super::rotmg::Empty>;

    fn logged_player(&self, o: ::grpc::RequestOptions, p: super::rotmg::EmailRequest) -> ::grpc::SingleResponse<super::rotmg::Empty>;

    fn authorize_player(&self, o: ::grpc::RequestOptions, p: super::rotmg::EmailRequest) -> ::grpc::SingleResponse<super::rotmg::Empty>;

    fn create_player(&self, o: ::grpc::RequestOptions, p: super::rotmg::CreatePlayerRequest) -> ::grpc::SingleResponse<super::rotmg::Player>;

    fn delete_player(&self, o: ::grpc::RequestOptions, p: super::rotmg::EmailRequest) -> ::grpc::SingleResponse<super::rotmg::Empty>;

    fn change_player(&self, o: ::grpc::RequestOptions, p: super::rotmg::Player) -> ::grpc::SingleResponse<super::rotmg::Empty>;

    fn server_info(&self, o: ::grpc::RequestOptions, p: super::rotmg::Empty) -> ::grpc::SingleResponse<super::rotmg::Server>;
}

// client

pub struct GameClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_KickPlayer: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rotmg::KickRequest, super::rotmg::Empty>>,
    method_ListPlayers: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rotmg::Empty, super::rotmg::ListPlayersResponse>>,
    method_GetPlayer: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rotmg::EmailRequest, super::rotmg::Player>>,
    method_BanPlayer: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rotmg::EmailRequest, super::rotmg::Empty>>,
    method_PardonPlayer: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rotmg::EmailRequest, super::rotmg::Empty>>,
    method_LoggedPlayer: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rotmg::EmailRequest, super::rotmg::Empty>>,
    method_AuthorizePlayer: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rotmg::EmailRequest, super::rotmg::Empty>>,
    method_CreatePlayer: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rotmg::CreatePlayerRequest, super::rotmg::Player>>,
    method_DeletePlayer: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rotmg::EmailRequest, super::rotmg::Empty>>,
    method_ChangePlayer: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rotmg::Player, super::rotmg::Empty>>,
    method_ServerInfo: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::rotmg::Empty, super::rotmg::Server>>,
}

impl ::grpc::ClientStub for GameClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        GameClient {
            grpc_client: grpc_client,
            method_KickPlayer: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/hagnix.Game/KickPlayer".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListPlayers: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/hagnix.Game/ListPlayers".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetPlayer: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/hagnix.Game/GetPlayer".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_BanPlayer: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/hagnix.Game/BanPlayer".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_PardonPlayer: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/hagnix.Game/PardonPlayer".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_LoggedPlayer: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/hagnix.Game/LoggedPlayer".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_AuthorizePlayer: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/hagnix.Game/AuthorizePlayer".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_CreatePlayer: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/hagnix.Game/CreatePlayer".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_DeletePlayer: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/hagnix.Game/DeletePlayer".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ChangePlayer: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/hagnix.Game/ChangePlayer".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ServerInfo: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/hagnix.Game/ServerInfo".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl Game for GameClient {
    fn kick_player(&self, o: ::grpc::RequestOptions, p: super::rotmg::KickRequest) -> ::grpc::SingleResponse<super::rotmg::Empty> {
        self.grpc_client.call_unary(o, p, self.method_KickPlayer.clone())
    }

    fn list_players(&self, o: ::grpc::RequestOptions, p: super::rotmg::Empty) -> ::grpc::SingleResponse<super::rotmg::ListPlayersResponse> {
        self.grpc_client.call_unary(o, p, self.method_ListPlayers.clone())
    }

    fn get_player(&self, o: ::grpc::RequestOptions, p: super::rotmg::EmailRequest) -> ::grpc::SingleResponse<super::rotmg::Player> {
        self.grpc_client.call_unary(o, p, self.method_GetPlayer.clone())
    }

    fn ban_player(&self, o: ::grpc::RequestOptions, p: super::rotmg::EmailRequest) -> ::grpc::SingleResponse<super::rotmg::Empty> {
        self.grpc_client.call_unary(o, p, self.method_BanPlayer.clone())
    }

    fn pardon_player(&self, o: ::grpc::RequestOptions, p: super::rotmg::EmailRequest) -> ::grpc::SingleResponse<super::rotmg::Empty> {
        self.grpc_client.call_unary(o, p, self.method_PardonPlayer.clone())
    }

    fn logged_player(&self, o: ::grpc::RequestOptions, p: super::rotmg::EmailRequest) -> ::grpc::SingleResponse<super::rotmg::Empty> {
        self.grpc_client.call_unary(o, p, self.method_LoggedPlayer.clone())
    }

    fn authorize_player(&self, o: ::grpc::RequestOptions, p: super::rotmg::EmailRequest) -> ::grpc::SingleResponse<super::rotmg::Empty> {
        self.grpc_client.call_unary(o, p, self.method_AuthorizePlayer.clone())
    }

    fn create_player(&self, o: ::grpc::RequestOptions, p: super::rotmg::CreatePlayerRequest) -> ::grpc::SingleResponse<super::rotmg::Player> {
        self.grpc_client.call_unary(o, p, self.method_CreatePlayer.clone())
    }

    fn delete_player(&self, o: ::grpc::RequestOptions, p: super::rotmg::EmailRequest) -> ::grpc::SingleResponse<super::rotmg::Empty> {
        self.grpc_client.call_unary(o, p, self.method_DeletePlayer.clone())
    }

    fn change_player(&self, o: ::grpc::RequestOptions, p: super::rotmg::Player) -> ::grpc::SingleResponse<super::rotmg::Empty> {
        self.grpc_client.call_unary(o, p, self.method_ChangePlayer.clone())
    }

    fn server_info(&self, o: ::grpc::RequestOptions, p: super::rotmg::Empty) -> ::grpc::SingleResponse<super::rotmg::Server> {
        self.grpc_client.call_unary(o, p, self.method_ServerInfo.clone())
    }
}

// server

pub struct GameServer;


impl GameServer {
    pub fn new_service_def<H : Game + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/hagnix.Game",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/hagnix.Game/KickPlayer".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.kick_player(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/hagnix.Game/ListPlayers".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_players(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/hagnix.Game/GetPlayer".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_player(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/hagnix.Game/BanPlayer".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.ban_player(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/hagnix.Game/PardonPlayer".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.pardon_player(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/hagnix.Game/LoggedPlayer".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.logged_player(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/hagnix.Game/AuthorizePlayer".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.authorize_player(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/hagnix.Game/CreatePlayer".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.create_player(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/hagnix.Game/DeletePlayer".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.delete_player(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/hagnix.Game/ChangePlayer".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.change_player(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/hagnix.Game/ServerInfo".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.server_info(o, p))
                    },
                ),
            ],
        )
    }
}
