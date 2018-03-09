// Copyright 2018 The Grin Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::thread;
use time::{self, now_utc};

use util::LOGGER;

// Will contain the crawler and modify the DNS records accordingly
pub fn connect_and_monitor(stop: Arc<AtomicBool>) {
    let _ = thread::Builder::new()
        .name("monitor".to_string())
        .spawn(move || {
            let mut prev = now_utc() - time::Duration::seconds(60);
            loop {
                let current_time = now_utc();

                if current_time - prev > time::Duration::seconds(20) {
                    // monitor dns records
                    // Example of monitoring
                    debug!(LOGGER, "monitoring");
                    prev = current_time;
                }
                thread::sleep(Duration::from_secs(1));

                if stop.load(Ordering::Relaxed) {
                    break;
                }
            }
        });
}
