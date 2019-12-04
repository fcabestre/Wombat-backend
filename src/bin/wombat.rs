use warp::filters::ws::WebSocket;
use warp::ws::Message;
use warp::{Filter, Future, Sink, Stream};

use wombat::stats_stream::stats_stream;

fn main() {
    pretty_env_logger::init();
    let routes = warp::path("stats")
        .and(warp::ws2())
        .map(|ws: warp::ws::Ws2| ws.on_upgrade(|websocket| new_connection(websocket)));
    warp::serve(routes).run(([0, 0, 0, 0], 3030));
}

fn new_connection(websocket: WebSocket) -> impl Future<Item = (), Error = ()> {
    websocket
        .send_all(
            stats_stream()
                .map_err(|_| -> warp::Error { unimplemented!("what else!") })
                .map(|stats| Message::text(serde_json::to_string(&stats).unwrap())),
        )
        .map_err(|_| ())
        .map(|_| ())
}
