use url::percent_encoding::percent_decode;

use qstring::QString;

pub fn get_param<'a>(data: &'a QString, param: &'a str) -> &'a str {
    match data.get(param) {
        None => "",
        Some(ip) => ip,
    }
}

pub fn get_param_as_bytes(data: &QString, param: &str) -> Option<Vec<u8>> {
    let param_as_str = get_param(data, param).as_bytes();

    percent_decode(param_as_str).if_any()
}
