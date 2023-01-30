mod channel_event;
mod pool_thread_channel;
mod done_thread_channel;

use channel_event::channel_event_test;
use pool_thread_channel::pool_channel_test;
use done_thread_channel::done_thread_channel_test;

fn main() {
    pool_channel_test();
}
