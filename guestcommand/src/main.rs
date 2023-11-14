cargo_component_bindings::generate!();

mod http14;
use crate::bindings::wasi::http::types::{Method, Scheme};

fn main() {
    let addr = "ipinfo.io";
    let res = http14::request(
        Method::Get,
        Scheme::Http,
        &addr,
        "/json",
        None,
        None,
    )
    .unwrap();
    eprintln!("res = {:?}", res.body);
}
