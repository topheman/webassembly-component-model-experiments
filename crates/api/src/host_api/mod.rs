#![allow(warnings)]

use wasmtime;

include!(concat!(env!("OUT_DIR"), "/host_api.rs"));
