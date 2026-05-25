use std::net::IpAddr;

use clap::Parser;

#[derive(Debug, Clone)]
pub struct Mapping {
  pub domain: String,
  pub ip: IpAddr,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
  #[arg(short, long)]
  pub ip: String,

  #[arg(short = 's', long, default_value = "simulacron.eu")]
  pub domain_suffix: String,

  #[arg(short, long, default_value_t = 53)]
  pub port: u16,

  #[arg(short, long)]
  pub response_ip: String,

  #[arg(short, long)]
  pub demonize: bool,

  #[arg(short, long)]
  pub update_allowed: bool,

  #[arg(value_parser = parse_domain_mapping, num_args = 0.., trailing_var_arg = true)]
  pub mappings: Vec<Mapping>,
}

fn parse_domain_mapping(s: &str) -> Result<Mapping, String> {
  let parts: Vec<&str> = s.splitn(2, '=').collect();
  if parts.len() != 2 {
    return Err(format!("Invalid format {s}. Expected domain=ip"));
  }
  let domain = parts[0].to_string();
  let ip: IpAddr = parts[1].parse()
    .map_err(|e| format!("Invalid IP {}: {}", parts[1], e))?;
  Ok(Mapping { domain, ip })
}

