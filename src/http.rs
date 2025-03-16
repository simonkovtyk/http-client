mod crate::utils;

use std;

pub struct RequestOptions<'a> {
  pub params: &'a std::collections::HashMap<&'a str, &'a str>,
  pub headers: &'a std::collections::HashMap<&'a str, &'a str>,
  pub timeout: u32,
  pub content_type: 
  pub ssl: bool
}

pub struct HttpClient {}

impl HttpClient {
   pub fn get(url: &str, options: RequestOptions) -> () {
       //let mut tcp_result: net::TcpStream = net::TcpStream::connect(url).unwrap();

       let params = utils::parse_params(options.params);

       let request_url = format!("{}{}", url, params);

       println!("{}", request_url);

       //tcp_result.write()
   } 
}
