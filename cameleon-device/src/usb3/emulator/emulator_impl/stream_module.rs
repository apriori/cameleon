use async_std::{
    prelude::*,
    sync::{Receiver, Sender},
};

use super::{
    device::Timestamp,
    shared_queue::SharedQueue,
    signal::{InterfaceSignal, StreamSignal},
};

// TODO: Implement stream module.
pub(super) struct StreamModule {
    _queue: SharedQueue<Vec<u8>>,
    _timestamp: Timestamp,

    enabled: bool,
}

impl StreamModule {
    pub(super) fn new(timestamp: Timestamp, queue: SharedQueue<Vec<u8>>) -> Self {
        Self {
            _timestamp: timestamp,
            _queue: queue,
            enabled: false,
        }
    }

    pub(super) async fn run(
        mut self,
        _signal_tx: Sender<InterfaceSignal>,
        mut signal_rx: Receiver<StreamSignal>,
    ) {
        while let Some(signal) = signal_rx.next().await {
            match signal {
                StreamSignal::_Enable => {
                    if self.enabled {
                        log::warn! {"receive event enable signal, but event module is already enabled"}
                    } else {
                        self.enabled = true;
                        log::info! {"event module is enabled"};
                    }
                }

                StreamSignal::Disable(_completed) => {
                    if self.enabled {
                        self.enabled = false;
                        log::info! {"event module is disenabled"};
                    } else {
                        log::warn! {"receive event disable signal, but event module is already disabled"}
                    }
                }

                StreamSignal::Shutdown => {
                    break;
                }
            }
        }
    }
}
