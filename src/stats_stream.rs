use std::str;
use std::time::Duration;

use futures::future::Future;
use futures::stream::poll_fn;
use futures::{try_ready, Stream};
use futures::{Async, Poll};
use tokio_timer::throttle::{Throttle, ThrottleError};

use crate::stats::Stats;

fn read_stats() -> Poll<Option<Stats>, tokio::io::Error> {
    trace!("polling file /proc/stats");
    let data = try_ready!(tokio::fs::read("/proc/stat").poll());
    let file_content = str::from_utf8(data.as_slice()).unwrap();
    let stats = Stats::parse(file_content);
    Ok(Async::Ready(Some(stats.unwrap())))
}

pub fn stats_stream() -> impl Stream<Item = Stats, Error = ThrottleError<tokio::io::Error>> {
    Throttle::new(poll_fn(read_stats), Duration::from_secs(1))
}
