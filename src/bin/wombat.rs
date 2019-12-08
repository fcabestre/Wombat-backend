use warp::filters::ws::WebSocket;
use warp::ws::Message;
use warp::{Filter, Future, Stream};

use futures::sync::mpsc;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use wombat::hguid::HGuid;
use wombat::stats_stream::stats_stream;

fn main() {
    let connections = Connections::new();
    let connections = warp::any().map(move || connections.clone());
    pretty_env_logger::init();
    let routes = warp::path("stats").and(warp::ws2()).and(connections).map(
        |ws: warp::ws::Ws2, connections: Connections| {
            let connections2 = connections.clone();
            warp::spawn(stats_stream().map_err(|_| ()).for_each(move |stats| {
                connections2.fan_out(Message::text(serde_json::to_string(&stats).unwrap()));
                Ok(())
            }));
            ws.on_upgrade(move |websocket| connections.add(websocket))
        },
    );
    warp::serve(routes).run(([0, 0, 0, 0], 3030));
}

#[derive(Clone)]
struct Connections {
    out_channels: Arc<Mutex<HashMap<HGuid, mpsc::UnboundedSender<Message>>>>,
}

impl Connections {
    fn new() -> Connections {
        Connections {
            out_channels: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn fan_out(&self, message: Message) {
        for (_, out_channel) in self.out_channels.lock().unwrap().iter() {
            out_channel.unbounded_send(message.clone()).unwrap();
        }
    }

    fn add(&self, websocket: WebSocket) -> impl Future<Item = (), Error = ()> {
        let channel_id = HGuid::new();
        let (tx, rx) = mpsc::unbounded();
        let (ws_tx, ws_rx) = websocket.split();

        warp::spawn(
            rx.map_err(|()| -> warp::Error { unreachable!("unbounded rx never errors") })
                .forward(ws_tx)
                .map(|_tx_rx| ())
                .map_err(|ws_err| eprintln!("websocket send error: {}", ws_err)),
        );

        println!("adding connection uid={:?}", channel_id);
        self.out_channels.lock().unwrap().insert(channel_id, tx);

        let out_channels2 = self.out_channels.clone();
        ws_rx
            .for_each(move |_| Ok(()))
            .then(move |result| {
                println!("removing connection uid={:?}", channel_id);
                out_channels2.lock().unwrap().remove(&channel_id);
                result
            })
            .map_err(move |e| {
                eprintln!("websocket error(uid={:?}): {}", channel_id, e);
            })
    }
}
