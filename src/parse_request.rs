#[derive(Debug, PartialEq)]
pub enum Name {
  Query { name: String },
  ResponseIp { ip: String, name: Option<String> },
}

/// Parse the name from the input string.
/// Format
/// - name:ip
/// - name
/// - ip
pub fn parse_name(name: String) -> Result<Name, String> {
  let elements = name.split(':').collect::<Vec<&str>>();
  if elements.len() == 1 {
    // check if it is an ip
    let ip = elements[0];
    return match ip.parse::<std::net::IpAddr>() {
      Ok(ip) => {
        if ip.is_ipv4() {
          Ok(Name::ResponseIp { ip: ip.to_string(), name: None })
        } else {
          Err("Only ipv4 is supported".to_string())
        }
      }
      Err(_) => Ok(Name::Query { name }),
    };
  } else if elements.len() == 2 {
    let ip = elements[1];

    return match ip.parse::<std::net::IpAddr>() {
      Ok(ip) => {
        if ip.is_ipv4() {
          Ok(Name::ResponseIp { ip: ip.to_string(), name: Some(elements[0].to_string()) })
        } else {
          Err("Only ipv4 is supported".to_string())
        }
      }
      Err(_) => Err("Invalid ip".to_string()),
    };
  }
  Err("Invalid name".to_string())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_name() {
    let name = "simulacron.eu".to_string();
    let result = parse_name(name);
    assert_eq!(result, Ok(Name::Query { name: "simulacron.eu".to_string() }));
  }

  #[test]
  fn test_parse_name_with_ip() {
    let name = "simulacron.eu:10.0.0.1".to_string();
    let result = parse_name(name);
    assert_eq!(result, Ok(Name::ResponseIp { ip: "10.0.0.1".to_string(), name: Some("simulacron.eu".to_string()) }));
  }

  #[test]
  fn test_parse_name_with_invalid_ip() {
    let name = "simulacron.eu:aha".to_string();
    let result = parse_name(name);
    assert_eq!(result, Err("Invalid ip".to_string()));
  }

  #[test]
  fn test_parse_name_with_empty_ip() {
    let name = "simulacron.eu:".to_string();
    let result = parse_name(name);
    assert_eq!(result, Err("Invalid ip".to_string()));
  }

  #[test]
  fn test_parse_ip_only() {
    let name = "10.0.0.1".to_string();
    let result = parse_name(name);
    assert_eq!(result, Ok(Name::ResponseIp { ip: "10.0.0.1".to_string(), name: None }));
  }
}
