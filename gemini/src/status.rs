use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref MESSAGES: HashMap<u8, &'static str> = init_message_list();
}

fn init_message_list() -> HashMap<u8, &'static str> {
    let mut hm = HashMap::new();

    hm.insert(10, "Input required");

    hm.insert(20, "Success");
    hm.insert(21, "Success (End of Certificate Session)");

    hm.insert(30, "Temporary Redirect");
    hm.insert(31, "Permanent Redirect");

    hm.insert(40, "Temporary Failure");
    hm.insert(41, "Server Unavailable");
    hm.insert(42, "CGI Error");
    hm.insert(43, "Proxy Error");
    hm.insert(44, "Slow Down (Rate Limited)");

    hm.insert(50, "Permanent Failure");
    hm.insert(51, "Not Found");
    hm.insert(52, "Gone");
    hm.insert(53, "Proxy Request Refused");
    hm.insert(59, "Bad Request");

    hm.insert(60, "Client Certificate Required");
    hm.insert(61, "Transient Certificate Required");
    hm.insert(62, "Authorized Certificate Required");
    hm.insert(63, "Certificate Not Accepted");
    hm.insert(64, "Future Certificate Rejected");
    hm.insert(65, "Expred Certificate Rejected");

    hm
}

pub fn get_message_for(code: u8) -> Option<String> {
    MESSAGES.get(&code).map(|f| f.to_string())
}
