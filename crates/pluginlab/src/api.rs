pub mod host_api {
    #![allow(warnings)]

    use wasmtime;

    include!(concat!(env!("OUT_DIR"), "/host_api.rs"));
}

pub mod plugin_api {
    #![allow(warnings)]

    use wasmtime;

    include!(concat!(env!("OUT_DIR"), "/plugin_api.rs"));
}
