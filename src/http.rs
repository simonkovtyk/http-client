use crate::utils;
use crate::enums;

use std;

pub struct RequestOptions<'a> {
  pub params: &'a std::collections::HashMap<&'a str, &'a str>,
  pub headers: &'a std::collections::HashMap<&'a str, &'a str>,
  pub timeout: u32,
  pub content_type: enums::ContentType,
  pub ssl: bool
}

pub struct HttpClient {}

impl HttpClient {
  pub fn get(url: &str, options: RequestOptions) -> () {
    //let mut tcp_result: net::TcpStream = net::TcpStream::connect(url).unwrap();

    // build url
    let params = utils::parse_params(options.params);
    let request_url = format!("{}{}", url, params);

    // detect request options
    let should_use_ssl: bool = options.ssl || utils::has_ssl_in_url(url); 

    println!("{}", request_url);

    //tcp_result.write()
  } 
}
