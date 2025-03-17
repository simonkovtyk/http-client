use crate::utils;
use crate::enums;

use std;
use serde::Serialize;

pub struct RequestOptions<'a> {
  pub params: std::option::Option<&'a std::collections::HashMap<&'a str, &'a str>>,
  pub headers: std::option::Option<&'a std::collections::HashMap<&'a str, &'a str>>,
  pub timeout: std::option::Option<u32>,
  pub content_type: std::option::Option<enums::ContentType>,
  pub ssl: std::option::Option<bool>
}

pub struct HttpClient {}

impl HttpClient {
  pub fn get(&self, url: &str, options: RequestOptions) -> &Self {
    //let mut tcp_result: net::TcpStream = net::TcpStream::connect(url).unwrap();

    // build url
    if !options.params.is_none() {
      let params = utils::parse_params(options.params.unwrap());
    }

    let request_url = format!("{}{}", url, params);

    // detect request options
    let should_use_ssl: bool = options.ssl || utils::has_ssl_in_url(url); 

    let method: &str = utils::get_method_str(&enums::Methods::GET);

    //format!("{}");

    //println!("{}", request_url);

    println!("{}", utils::get_relative_url(url));

    //tcp_result.write()
    
    return self;
  }

  pub fn json(&self, data: Serialize) -> &Self {
    

    return self;
  }
}
