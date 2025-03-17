use crate::constants;
use crate::enums;

use std;

pub fn parse_params(params: &std::collections::HashMap<&str, &str>) -> String {
  let params_iter = params.into_iter();

  if params_iter.clone().count() == 0 {
    return String::from("");
  }

  let mut params_string: String = String::from("");

  for (key, value) in params_iter {
    params_string.push_str(
      &format!("&{}={}", key, value)
    );
  }

  return params_string;
}

pub fn get_method_str(method: &'static enums::Methods) -> &'static str {
  match method {
    enums::Methods::GET => return "GET",
    enums::Methods::POST => return "POST",
    enums::Methods::PUT => return "PUT",
    enums::Methods::DELETE => return "DELETE",
    enums::Methods::PATCH => return "PATCH",
    enums::Methods::HEAD => return "HEAD",
    enums::Methods::OPTIONS => return "OPTIONS"
  }
}

pub fn parse_headers(headers: &std::collections::HashMap<&str, &str>) -> String {
  let headers_iter = headers.into_iter();

  if headers_iter.clone().count() == 0 {
    return String::from("");
  }

  let mut headers_string: String = String::from("");

  for (key, value) in headers_iter {
    headers_string.push_str(
      &format!("{}: {}{}", key, value, constants::LINEBREAK)
    );
  }

  return headers_string;
}

pub fn has_ssl_in_url(url: &str) -> bool {
  return url.starts_with("https");
}

pub fn serialize_json<T: serde::ser::Serialize>(data: &T) -> String {
  return serde_json::to_string(data).unwrap();
}

pub fn get_relative_url(url: &str) -> String {
  let byte_index_of_first_slash: usize = url.find("/").unwrap();

  return url.to_string()[byte_index_of_first_slash + 1..].to_string();
}
