extern crate protobuf;
extern crate grpcio;
extern crate futures;

#[macro_use]
extern crate log;

mod grpc;

use std::sync::Arc;
use std::io::Read;
use std::{io, thread};

use futures::Future;
use futures::sync::oneshot;

use protobuf::RepeatedField;

use grpc::artist::{ArtistList, ArtistInfo};
use grpc::query::Query;
use grpc::query_grpc::{self, QueryService};
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

#[derive(Clone)]
struct QueryServicer;

impl QueryService for QueryServicer {
    fn query_playlists(&self, ctx: ::grpcio::RpcContext, req: grpc::query::Query, sink: ::grpcio::UnarySink<grpc::playlist::PlaylistList>) {
    }

    fn query_songs(&self, ctx: ::grpcio::RpcContext, req: grpc::query::Query, sink: ::grpcio::UnarySink<grpc::song::SongList>) {

    }

    fn query_albums(&self, ctx: ::grpcio::RpcContext, req: grpc::query::Query, sink: ::grpcio::UnarySink<grpc::album::AlbumList>) {

    }

    fn query_artists(&self, ctx: RpcContext, req: Query, sink: UnarySink<ArtistList>) {
        let artists = {
            let msg = req.get_query();
            let id = req.get_id();
            let mut artists = Vec::new();

            let mut sap = ArtistInfo::new();
            sap.set_id(1);
            sap.set_title("Sapphire".to_owned());
            artists.push(sap);

            artists
        };
        let mut resp = ArtistList::new();
        resp.set_artists(RepeatedField::from_vec(artists));
        let f = sink.success(resp)
            .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }

    fn query_all(&self, ctx: ::grpcio::RpcContext, req: grpc::query::Query, sink: ::grpcio::UnarySink<grpc::query::AggregateList>) {

    }
}


fn main() {
    let env = Arc::new(Environment::new(1));
    let service = query_grpc::create_query_service(QueryServicer);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50051)
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
