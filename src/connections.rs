use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use futures::sync::mpsc;
use warp::filters::ws::WebSocket;
use warp::ws::Message;
use warp::{Future, Stream};

use crate::hguid::HGuid;

#[derive(Clone)]
pub struct Connections {
    out_channels: Arc<RwLock<HashMap<HGuid, mpsc::UnboundedSender<Message>>>>,
}

impl Connections {
    pub fn new() -> Connections {
        Connections {
            out_channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn dispatch_message(&self, message: Message) {
        for (channel_id, out_channel) in self.out_channels.read().unwrap().iter() {
            trace!("sending message to connection uid={:?}", channel_id);
            out_channel.unbounded_send(message.clone()).unwrap();
        }
    }

    pub fn add(&self, websocket: WebSocket) -> impl Future<Item = (), Error = ()> {
        let channel_id = HGuid::new();
        let (tx, rx) = mpsc::unbounded();
        let (ws_tx, ws_rx) = websocket.split();

        warp::spawn(
            rx.map_err(|()| -> warp::Error { unreachable!("unbounded rx never errors") })
                .forward(ws_tx)
                .map(|_tx_rx| ())
                .map_err(|ws_err| error!("websocket send error: {}", ws_err)),
        );

        trace!("adding connection uid={:?}", channel_id);
        self.out_channels.write().unwrap().insert(channel_id, tx);

        let out_channels2 = self.out_channels.clone();
        ws_rx
            .for_each(move |_| Ok(()))
            .then(move |result| {
                trace!(
                    "dropping connection uid={:?} result={:?}",
                    channel_id,
                    result
                );
                out_channels2.write().unwrap().remove(&channel_id);
                result
            })
            .map_err(move |e| {
                error!("websocket error(uid={:?}): {}", channel_id, e);
            })
    }
}
