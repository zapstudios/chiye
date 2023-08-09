use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;

fn perform_get_request(url: &str) -> Result<String, std::io::Error> {
    let mut stream = TcpStream::connect(url)?;

    let request = format!("GET / HTTP/1.1\r\nHost: {}\r\n\r\n", url);
    stream.write_all(request.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    Ok(response)
}

fn perform_post_request(url: &str, data: &str) -> Result<String, std::io::Error> {
    let mut stream = TcpStream::connect(url)?;

    let request = format!(
        "POST / HTTP/1.1\r\nHost: {}\r\nContent-Length: {}\r\n\r\n{}",
        url,
        data.len(),
        data
    );
    stream.write_all(request.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    Ok(response)
}

fn parse_json(json: &str) -> Result<HashMap<String, String>, &'static str> {
    let mut data = HashMap::new();
    let mut in_key = false;
    let mut in_value = false;
    let mut key = String::new();
    let mut value = String::new();

    for c in json.chars() {
        match c {
            '"' if in_key => in_key = false,
            '"' if in_value => in_value = false,
            '"' => in_key = true,
            ':' => in_value = true,
            ',' if in_value => {
                in_value = false;
                data.insert(key.clone(), value.clone());
                key.clear();
                value.clear();
            }
            _ if in_key => key.push(c),
            _ if in_value => value.push(c),
            _ => {}
        }
    }

    if !key.is_empty() && !value.is_empty() {
        data.insert(key, value);
    }

    if data.is_empty() {
        Err("Failed to parse JSON")
    } else {
        Ok(data)
    }
}

fn main() -> Result<(), std::io::Error> {
    // Example/Test URL
    let rest_response = perform_get_request("https://cat-fact.herokuapp.com/facts/")?;

    let parsed_json = parse_json(&rest_response);
    match parsed_json {
        Ok(data) => {
            println!("Parsed JSON:");
            for (key, value) in data.iter() {
                println!("{}: {}", key, value);
            }
        }
        Err(error) => {
            println!("Error parsing JSON: {}", error);
        }
    }

    Ok(())
}
