#[macro_use]
extern crate log;

use warp::ws::Message;
use warp::{Filter, Future, Reply, Stream};

use warp::filters::BoxedFilter;
use wombat::connections::Connections;
use wombat::stats_stream::stats_stream;

fn main() {
    pretty_env_logger::init();

    let connections = Connections::new();
    let connections2 = connections.clone();
    let (addr, warp_server) =
        warp::serve(route_definition(connections)).bind_ephemeral(([0, 0, 0, 0], 3030));
    trace!("listening on {:?}", addr);
    tokio::run(warp_server.join(stats_producer(connections2)).map(|_| ()))
}

fn route_definition(connections: Connections) -> BoxedFilter<(impl Reply,)> {
    let with_connections = warp::any().map(move || connections.clone());
    warp::path("stats")
        .and(warp::ws2())
        .and(with_connections)
        .map(|ws: warp::ws::Ws2, connections: Connections| {
            ws.on_upgrade(move |websocket| connections.add(websocket))
        })
        .boxed()
}

fn stats_producer(connections: Connections) -> impl Future<Item = (), Error = ()> {
    stats_stream().map_err(|_| ()).for_each(move |stats| {
        connections.dispatch_message(Message::text(serde_json::to_string(&stats).unwrap()));
        Ok(())
    })
}
