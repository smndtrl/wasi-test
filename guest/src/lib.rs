cargo_component_bindings::generate!();


use crate::bindings::exports::smndtrl::simple::function::Guest;
use crate::bindings::wasi::http::types::{Method, Scheme};
mod http14;

pub struct Component;

impl Guest for Component {
    fn map(_: Vec<u8>) -> Vec<u8> { 
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

        return res.body;
     }
}