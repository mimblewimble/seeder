# grin-seeder
DNS seed server for Grin

## Building

### Build Prerequisites

In order to compile and run Grin on your machine, you should have installed:

* <b>Git</b> - to clone the repository
* <b>Rust</b> - 1.21.0 or greater via [Rustup](https://www.rustup.rs/) - Can be installed via your package manager or manually via the following commands:
```
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
```

### Build Instructions (Linux/Unix)


#### Clone Grin

```
git clone https://github.com/mimblewimble/seeder.git
```

#### Build Grin
```sh
cd grin-seeder
git checkout 0.1
cargo build
```

## Usage

Let's say you want to run your dns seed on seed.example.com.
In that case, you'll need an authorative NS record in example.com's domain record, poiting to, for example, vps.example.com:

```
$ dig -t NS seed.example.com

;; ANSWER SECTION
seed.example.com.   86400    IN      NS     vps.example.com.
```

Then, on the system vps.example.com, you can run grin-seeder:
```sh
grin-seeder -h dnsseed.example.com -n vps.example.com
```

If you want the DNS server to report SOA records, please provide an
e-mail address (with the @ part replaced by .) using -m.


## Running as non-root

Typically, you'll need root privileges to listen to port 53 (name service).

One solution is using an iptables rule (Linux only) to redirect it to
a non-privileged port:

```sh
$ iptables -t nat -A PREROUTING -p udp --dport 53 -j REDIRECT --to-port 5353
```

If properly configured, this will allow you to run grin-seeder in userspace, using
the -p 5353 option.
