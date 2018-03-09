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

use std::io;
use std::net::Ipv4Addr;

/// Configuration for the DNS server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNSConfig {
    pub origin: String,
    pub host: Ipv4Addr,
    pub port: u16,
    pub seeds: Vec<Ipv4Addr>,
    pub primary_ns: String,
    pub email: String,
}

/// Default configuration for the DNS server.
impl Default for DNSConfig {
    fn default() -> DNSConfig {
        let ipaddr = "127.0.0.1".parse().unwrap();
        let seeds = vec![
            Ipv4Addr::new(192, 241, 168, 77),
            Ipv4Addr::new(109, 74, 202, 16),
            Ipv4Addr::new(174, 138, 116, 153),
            Ipv4Addr::new(46, 4, 91, 48),
            Ipv4Addr::new(51, 15, 219, 12),
        ];
        DNSConfig {
            origin: String::from("example.com."),
            host: ipaddr,
            port: 53,
            seeds: seeds,
            primary_ns: String::from("sns.dns.icann.org."),
            email: String::from("admin.example.com"),
        }
    }
}

// Error Handling
#[derive(Debug)]
pub enum Error {
    Connection(io::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Connection(e)
    }
}
