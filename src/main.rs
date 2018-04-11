mod http;

fn main() {
    println!("Server Started");
    http::start_http_server(String::from("0.0.0.0"), 8001);
}
