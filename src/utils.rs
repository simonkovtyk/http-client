use std;

pub fn parse_params<'a>(params: &std::collections::HashMap<&str, &str>) -> String {
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

pub fn parse_headers<'a>(headers: &std::collections::HashMap<&str, &str>) -> String {
    let headers_iter = headers.into_iter();

    if headers_iter.clone().count() == 0 {
        return String::from("");
    }

    let mut headers_string: String = String::from("");

    for (key, value) in headers_iter {
        headers_string.push_str(
            &format!("{}: {}\n", key, value)
        );
    }

    return headers_string;
}
