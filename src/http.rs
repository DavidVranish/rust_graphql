extern crate tiny_http;

pub fn start_http_server(arg_ip: String, arg_port: u32) -> () {
    let loc_address_string: String = format!("{}:{}", arg_ip, arg_port.to_string());

    let loc_server = tiny_http::Server::http(loc_address_string).unwrap();

    loop {
        let loc_request = match loc_server.recv() {
            Ok(rq) => rq,
            Err(e) => { println!("error: {}", e); break }
        };

        handle_request(loc_request);
    }
}

fn handle_request(arg_request: tiny_http::Request) -> () {
    println!("{}", arg_request.method());
    let loc_response_string = String::from("woooooo!");
    let loc_reponse = tiny_http::Response::from_string(loc_response_string);

    let _ = arg_request.respond(loc_reponse);
}
