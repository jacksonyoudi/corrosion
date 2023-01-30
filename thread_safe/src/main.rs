extern crate core;
extern crate core;

mod channel_event;
mod pool_thread_channel;
use channel_event::channel_event_test;
use pool_thread_channel::pool_channel_test;

fn main() {
    pool_channel_test();
}
