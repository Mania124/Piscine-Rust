pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown =>  server.unwrap().to_string(),
        Security::Message =>  server.expect("ERROR: program stops").to_string(),
        Security::Warning => server.unwrap_or("WARNING: check the server").to_string(),
        Security::NotFound => server.map(String::from).unwrap_or_else(|url|format!("Not found: {}", url)),
        Security::UnexpectedUrl => server.unwrap_err().to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
        let result = fetch_data(Ok("server1.com"), Security::Warning);
        assert_eq!(result, "server1.com".to_string());
        let res = fetch_data(Err("server.com"), Security::Warning);
        assert_eq!(res, "WARNING: check the server".to_string());
    }
}
