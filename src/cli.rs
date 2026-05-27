use std::net::IpAddr;

use clap::Parser;

#[derive(Debug, Clone)]
pub struct Mapping {
  pub domain: String,
  pub ip: IpAddr,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  #[arg(short, long)]
  pub ip: String,

  /// currently unused
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

  #[arg(short, long)]
  pub verbose: bool,

  /// Domain to IP mappings in the format domain=ip
  #[arg(value_parser = parse_domain_mapping, num_args = 0.., trailing_var_arg = true)]
  pub mappings: Vec<Mapping>,
}

fn parse_domain_mapping(s: &str) -> Result<Mapping, String> {
  let parts: Vec<&str> = s.splitn(2, '=').collect();
  if parts.len() != 2 {
    return Err(format!("Invalid format {s}. Expected domain=ip"));
  }
  let domain = parts[0].to_string();
  if domain.is_empty() {
    return Err("Domain name cannot be empty in a mapping".to_string());
  }
  let ip: IpAddr = parts[1].parse().map_err(|e| format!("Invalid IP {}: {}", parts[1], e))?;
  Ok(Mapping { domain, ip })
}

#[cfg(test)]
mod tests_mappings {
  use super::*;

  // Helper function to simulate CLI arguments
  fn parse_args(args: &[&str]) -> Result<Cli, clap::Error> {
    // "test_bin" simulates the executable name (argv[0])
    let mut full_args = vec!["test_bin", "-i 192.168.0.10", "-r 192.168.0.100"];
    full_args.extend_from_slice(args);
    Cli::try_parse_from(full_args)
  }

  #[test]
  fn test_optional_mappings_empty() {
    let res = parse_args(&["-v"]);
    assert!(res.is_ok());

    let cli = res.unwrap();
    assert!(cli.verbose);
    assert!(cli.mappings.is_empty());
  }

  #[test]
  fn test_single_mapping() {
    let res = parse_args(&["example.com=1.1.1.1"]);
    assert!(res.is_ok());

    let cli = res.unwrap();
    assert_eq!(cli.mappings.len(), 1);
    assert_eq!(cli.mappings[0].domain, "example.com");
    assert_eq!(cli.mappings[0].ip.to_string(), "1.1.1.1");
  }

  #[test]
  fn test_multiple_mappings() {
    let res = parse_args(&["example.com=1.1.1.1", "google.com=8.8.8.8"]);
    assert!(res.is_ok());

    let cli = res.unwrap();
    assert_eq!(cli.mappings.len(), 2);
    assert_eq!(cli.mappings[1].domain, "google.com");
    assert_eq!(cli.mappings[1].ip.to_string(), "8.8.8.8");
  }

  #[test]
  fn test_invalid_format_missing_equals() {
    let res = parse_args(&["example.com1.1.1.1"]);
    assert!(res.is_err());

    let err = res.unwrap_err();
    // Verifies our custom error message is inside clap's error output
    assert!(err.to_string().contains("Invalid format"));
  }

  #[test]
  fn test_invalid_ip_address() {
    let res = parse_args(&["example.com=999.999.999.999"]);
    assert!(res.is_err());

    let err = res.unwrap_err();
    assert!(err.to_string().contains("Invalid IP"));
  }

  #[test]
  fn test_empty_domain() {
    let res = parse_args(&["=1.1.1.1"]);
    assert!(res.is_err());

    let err = res.unwrap_err();
    assert!(err.to_string().contains("Domain name cannot be empty"));
  }
}
