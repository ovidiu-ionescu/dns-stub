mod parse_request;

use clap::Parser;
use log::{debug, info, log_enabled, warn, Level};
use std::io::Write;
use std::net::UdpSocket;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  ip: String,

  #[arg(short = 's', long, default_value = "simulacron.eu")]
  domain_suffix: String,

  #[arg(short, long, default_value_t = 53)]
  port: u16,

  #[arg(short, long)]
  response_ip: String,

  #[arg(short, long)]
  demonize: bool,

  #[arg(short, long)]
  update_allowed: bool,
}

fn main() {
  let args = Args::parse();
  env_logger::init();
  server(&args.ip, args.port, &args.response_ip, &args.domain_suffix);
}

fn write(n: u16, vec: &mut [u8], index: usize) {
  let be = n.to_be_bytes();
  vec[index] = be[0];
  vec[index + 1] = be[1];
}

fn read(buf: &[u8], start: usize) -> usize {
  u16::from_be_bytes([buf[start], buf[start + 1]]) as usize
}

fn extract_name(buf: &[u8], start: usize) -> String {
  let mut res = String::new();
  let mut crt = start;
  let mut len = buf[start] as usize;
  while len > 0 {
    if len >= 192 {
      crt = u16::from_be_bytes([(len - 192) as u8, buf[crt + 1]]) as usize;
      len = buf[crt] as usize;
    }
    res.push_str(std::str::from_utf8(&buf[crt + 1..=crt + len]).unwrap());
    crt += len + 1;
    len = buf[crt] as usize;
    if len != 0 {
      res.push('.');
    }
  }
  res
}

fn server(ip: &str, port: u16, initial_response_ip: &str, domain_suffix: &str) {
  let address = format!("{ip}:{port}");
  info!("Starting server on {}", address);
  let server_socket = UdpSocket::bind(address).expect("Could not bind server socket");
  //drop_capabilities().expect("Failed to drop capabilities");
  let mut counter = 0;
  let mut default_response_ip = initial_response_ip.to_string();

  let mut database = std::collections::HashMap::<String, String>::new();

  loop {
    let mut buf = [0; 512];
    let (amt, src) = server_socket.recv_from(&mut buf).expect("Failed to receive packet");
    debug!("Received {} bytes from {}", amt, src);

    // write request buffer to file
    if log_enabled!(Level::Debug) {
      let mut file = std::fs::File::create(format!("request-{}.bin", counter)).unwrap();
      file.write_all(&buf[..amt]).unwrap();
    }

    // toggle the QR bit
    buf[2] ^= 0b10000000;

    // get number of additional records
    let additional_records = read(&buf, 10);
    // set additional records to 0
    write(0, &mut buf, 10);
    if additional_records > 0 {
      debug!("Additional records: {}", additional_records);
    }

    let name = extract_name(&buf, 12);
    debug!("Name: {}", name);
    // for adding to the end of the buffer
    let mut ptr = 12 + name.len() + 6;
    debug!("Payload should end at {}", ptr);

    // get the query type
    let qtype = read(&buf, ptr - 4);
    // for better logging, decide now if we are going to respond
    let will_respond = (qtype == 1 || qtype == 23) && name.ends_with(domain_suffix);
    info!(
      "Query for {} of type {} ({}), additional records: {} | will respond: {will_respond}",
      name,
      qtype,
      if qtype == 1 { "A" } else { "other" },
      additional_records
    );
    let parsed_name = parse_name(name);
    let response_ip = match qtype {
      // we are changing the response IP
      // if just the IP is sent, we will use it as the default response IP
      23 => match parsed_name {
        Ok(Name::ResponseIp { ip, name: None }) => {
          default_response_ip = ip;
          info!("Setting response IP to {}", default_response_ip);
          &default_response_ip
        }
        Ok(Name::ResponseIp { ip, name: Some(name) }) => {
          info!("Setting response IP for 「{}」 to {}", name, ip);
          let r = ip.clone();
          database.insert(name, ip);
          &r.clone()
        }
        Err(e) => {
          warn!("Failed to parse name: {}", e);
          &default_response_ip
        }
        _ => &default_response_ip,
      },
      // Regular query
      1 => match parsed_name {
        Ok(Name::Query { name }) => {
          if let Some(ip) = database.get(&name) {
            ip
          } else {
            &default_response_ip
          }
        }
        Err(e) => {
          warn!("Failed to parse name {e}");
          &default_response_ip
        }
        _ => &default_response_ip,
      },

      _ => &default_response_ip,
    };

    match qtype {
      1 | 23 => {
        info!("Responding with IP {response_ip}");
        // set the answer count to 1
        buf[7] = 1;

        // set AA bit
        buf[2] |= 0b00000100;

        // name is a pointer to the name in the question
        // 0xC00C is the offset of the name in the buffer
        buf[ptr] = 0xC0;
        buf[ptr + 1] = 0x0C;
        ptr += 2;
        // TYPE A
        write(1, &mut buf, ptr);
        ptr += 2;
        // CLASS IN
        write(1, &mut buf, ptr);
        ptr += 2;
        // TTL one minute
        write(00, &mut buf, ptr);
        ptr += 2;
        write(60, &mut buf, ptr);
        ptr += 2;
        // RDLENGTH 4 (IP address)
        write(4, &mut buf, ptr);
        ptr += 2;
        // RDATA
        response_ip.split('.').for_each(|s| {
          let n = s.parse::<u8>().unwrap();
          buf[ptr] = n;
          ptr += 1;
        });
      }
      _ => {
        debug!("Unsuported query type, set answer count to 0");
        buf[7] = 0;
      }
    }

    if log_enabled!(Level::Debug) {
      let mut file = std::fs::File::create(format!("response-{}.bin", counter)).unwrap();
      file.write_all(&buf[..ptr]).unwrap();
    }
    counter += 1;

    server_socket.send_to(&buf[..ptr], src).expect("Failed to send packet");
  }
}

use crate::parse_request::{parse_name, Name};
use libc::c_int;
use libc::{prctl, PR_CAPBSET_DROP};

pub const CAP_NET_BIND_SERVICE: c_int = 10;

/// This will only work if the process has CAP_SETPCAP capability.
/// Dropping capabilities also needs to be allowed, it's not implicit
#[allow(dead_code)]
fn drop_capabilities() -> Result<(), String> {
  let ret = unsafe { prctl(PR_CAPBSET_DROP, CAP_NET_BIND_SERVICE, 0, 0, 0) };
  if ret != 0 {
    return Err(format!("Failed to drop capabilities: {}", ret));
  }
  Ok(())
}
