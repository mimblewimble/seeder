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

extern crate grin_seeder_config as config;
extern crate grin_seeder_dns_server as dns_server;
extern crate grin_seeder_util as util;

extern crate clap;
extern crate daemonize;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
extern crate time;

mod monitor;
mod server;
mod types;

use clap::{App, Arg, ArgMatches, SubCommand};
use daemonize::Daemonize;
use std::thread;
use std::time::Duration;
use std::env::current_dir;

use server::Server;
use config::GlobalConfig;
use util::{init_logger, LoggingConfig, LOGGER};

fn main() {
    // First, load a global config object,
    // then modify that object with any switches
    // found so that the switches override the
    // global config file

    // This will return a global config object,
    // which will either contain defaults for all // of the config structures or a
    // configuration
    // read from a config file

    let mut global_config = GlobalConfig::new(None).unwrap_or_else(|e| {
        panic!("Error parsing config file: {}", e);
    });

    if global_config.using_config_file {
        // initialise the logger
        init_logger(global_config.members.as_mut().unwrap().logging.clone());
        info!(
            LOGGER,
            "Using configuration file at: {}",
            global_config
                .config_file_path
                .clone()
                .unwrap()
                .to_str()
                .unwrap()
        );
    } else {
        init_logger(Some(LoggingConfig::default()));
    }

    let args = App::new("Grin-seeder")
		.version("0.1")
		.author("Quentin le Sceller")
		.about("DNS Seed server and crawler for Grin.")

		// specification of all the server commands and options
	                .arg(Arg::with_name("host")
	                     .short("h")
	                     .long("host")
	                     .help("Host to run Grin-seeder DNS server on")
	                     .takes_value(true))
	                .arg(Arg::with_name("name")
	                     .short("n")
	                     .long("name")
	                     .help("Hostname of the DNS server host")
	                     .takes_value(true))
					.arg(Arg::with_name("port")
	 	                     .short("p")
	 	                     .long("port")
	 	                     .help("Port for the DNS server")
	 	                     .takes_value(true))
	                .arg(Arg::with_name("email")
	                     .short("m")
	                     .long("email")
	                     .help("Email for the SOA Records")
	                     .takes_value(true))
	                .arg(Arg::with_name("ns")
	                     .short("d")
	                     .long("ns")
	                     .help("Hostname of the primary name server")
						 .takes_value(true))
	                .subcommand(SubCommand::with_name("start")
	                            .about("Start Grin-seeder as a daemon"))
	                .subcommand(SubCommand::with_name("stop")
	                            .about("Stop Grin-seeder daemon"))
	                .subcommand(SubCommand::with_name("run")
	                            .about("Run Grin-seeder in this console"))
		.get_matches();

    server_command(&args, global_config);

    fn server_command(server_args: &ArgMatches, global_config: GlobalConfig) {
        info!(LOGGER, "Starting the Grin-seeder server...");

        // just get defaults from the global config
        let mut server_config = global_config.members.unwrap().dns_server;

        if let Some(host) = server_args.value_of("host") {
            server_config.host = host.parse().unwrap();
        }
        if let Some(port) = server_args.value_of("port") {
            server_config.port = port.parse().unwrap();
        }
        if let Some(name) = server_args.value_of("name") {
            server_config.origin = name.parse().unwrap();
        }
        if let Some(email) = server_args.value_of("email") {
            server_config.email = email.parse().unwrap();
        }
        if let Some(ns) = server_args.value_of("ns") {
            server_config.primary_ns = ns.parse().unwrap();
        }

        // start the server in the different run modes (interactive or daemon)
        match server_args.subcommand() {
            ("run", _) => {
                Server::start(server_config).unwrap();
            }
            ("start", _) => {
                let daemonize = Daemonize::new()
                    .pid_file("/tmp/grin-seeder.pid")
                    .chown_pid_file(true)
                    .working_directory(current_dir().unwrap())
                    .privileged_action(move || {
                        Server::start(server_config.clone()).unwrap();
                        loop {
                            thread::sleep(Duration::from_secs(60));
                        }
                    });
                match daemonize.start() {
                    Ok(_) => info!(LOGGER, "Grin-seeder successfully started."),
                    Err(e) => error!(LOGGER, "Error starting: {}", e),
                }
            }
            ("stop", _) => {
                println!("TODO. Just 'kill $pid' for now. Maybe /tmp/grin-seeder.pid is $pid")
            }
            (cmd, _) => {
                println!(":: {:?}", server_args);
                panic!(
                    "Unknown server command '{}', use 'grin-seeder help' for details",
                    cmd
                );
            }
        }
    }
}
