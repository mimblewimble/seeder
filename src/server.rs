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
use std::{thread, time};

use dns_server;
use monitor;
use types::Error;
use dns_server::DNSConfig;

/// Grin server holding internal structures.
pub struct Server {
    /// server config
    pub config: DNSConfig,
    /// handle to our network server
    pub dns: Arc<dns_server::Server>,
    stop: Arc<AtomicBool>,
}

impl Server {
    /// Instantiates and starts a new server.
    pub fn start(config: DNSConfig) -> Result<(), Error> {
        let serv = Server::new(config)?;
        loop {
            thread::sleep(time::Duration::from_secs(1));
            if serv.stop.load(Ordering::Relaxed) {
                return Ok(());
            }
        }
    }

    pub fn new(config: DNSConfig) -> Result<Server, Error> {
        let stop = Arc::new(AtomicBool::new(false));

        let dns_server = Arc::new(dns_server::Server::new(config.clone(), stop.clone()).unwrap());

        monitor::connect_and_monitor(stop.clone());

        let dns_inner = dns_server.clone();
        let _ = thread::Builder::new()
            .name("dns-server".to_string())
            .spawn(move || dns_inner.start());

        Ok(Server {
            config: config,
            dns: dns_server,
            stop: stop,
        })
    }
}
