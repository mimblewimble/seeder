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

use std::collections::BTreeMap;
use std::io;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use trust_dns::rr::*;
use trust_dns::rr::rdata::*;
use trust_dns_server::ServerFuture;
use trust_dns_server::authority::{Authority, Catalog, ZoneType};

use types::{DNSConfig, Error};
use util::LOGGER;

/// DNS server implementation
pub struct Server {
    config: DNSConfig,
    stop: Arc<AtomicBool>,
}

unsafe impl Sync for Server {}
unsafe impl Send for Server {}

impl Server {
    /// Creates a new idle dns server with empty catalog
    pub fn new(config: DNSConfig, stop: Arc<AtomicBool>) -> Result<Server, Error> {
        Ok(Server {
            config: config.clone(),
            stop: stop,
        })
    }

    pub fn start(&self) -> Result<thread::JoinHandle<()>, io::Error> {
        let addr = SocketAddr::V4(SocketAddrV4::new(self.config.host, self.config.port));
        let udp_socket = UdpSocket::bind(&addr).unwrap();
        let ipaddr = udp_socket.local_addr().unwrap();
        info!(LOGGER, "Starting DNS server on {}", ipaddr);
        let stop = self.stop.clone();

        // Creating authority and catalog
        let authority = self.create_default_authority();
        let catalog = Server::get_catalog(authority);

        thread::Builder::new()
            .name("dns:udp:server".to_string())
            .spawn(move || {
                let mut server = ServerFuture::new(catalog).expect("new udp server failed");
                server.register_socket(udp_socket);

                while !stop.load(Ordering::Relaxed) {
                    server.tokio_core().turn(Some(Duration::from_millis(10)));
                }
            })
    }

    pub fn stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }

    pub fn get_catalog(authority: Authority) -> Catalog {
        let mut catalog: Catalog = Catalog::new();
        catalog.upsert(authority.origin().clone().into(), authority);
        catalog
    }

    fn create_default_authority(&self) -> Authority {
        let origin: Name = Name::parse(&self.config.origin, None).unwrap();
        let mut authority: Authority = Authority::new(
            origin.clone(),
            BTreeMap::new(),
            ZoneType::Master,
            false,
            true,
        );
        // example.com.		3600	IN	SOA	sns.dns.icann.org. noc.dns.icann.org. 2015082403 7200 3600 1209600 3600
        authority.upsert(
            Record::new()
                .set_name(origin.clone())
                .set_ttl(3600)
                .set_rr_type(RecordType::SOA)
                .set_dns_class(DNSClass::IN)
                .set_rdata(RData::SOA(SOA::new(
                    Name::parse(&self.config.primary_ns, None).unwrap(),
                    Name::parse(&self.config.email, None).unwrap(),
                    2015082403,
                    7200,
                    3600,
                    1209600,
                    3600,
                )))
                .clone(),
            0,
        );

        authority.upsert(
            Record::new()
                .set_name(origin.clone())
                .set_ttl(86400)
                .set_rr_type(RecordType::NS)
                .set_dns_class(DNSClass::IN)
                .set_rdata(RData::NS(
                    Name::parse(&self.config.primary_ns, None).unwrap(),
                ))
                .clone(),
            0,
        );

        for seed in self.config.clone().seeds {
            authority.upsert(Server::create_a_record(origin.clone(), 86400, seed), 0);
        }
        authority
    }

    fn create_a_record(origin: Name, ttl: u32, ip_address: Ipv4Addr) -> Record {
        Record::new()
            .set_name(origin.clone())
            .set_ttl(ttl)
            .set_rr_type(RecordType::A)
            .set_dns_class(DNSClass::IN)
            .set_rdata(RData::A(ip_address))
            .clone()
    }

    #[allow(unused)]
    fn create_aaaa_record(origin: Name, ip_address: Ipv6Addr) -> Record {
        Record::new()
            .set_name(origin.clone())
            .set_ttl(86400)
            .set_rr_type(RecordType::AAAA)
            .set_dns_class(DNSClass::IN)
            .set_rdata(RData::AAAA(ip_address))
            .clone()
    }
}
