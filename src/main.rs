#![crate_name="acronymy"]
#![crate_type = "bin"]

extern crate libc;
extern crate time;
extern crate url;
extern crate capnp;
extern crate capnp_rpc;
extern crate fdstream;
extern crate sqlite3;

pub mod grain_capnp {
  include!(concat!(env!("OUT_DIR"), "/grain_capnp.rs"));
}

pub mod util_capnp {
  include!(concat!(env!("OUT_DIR"), "/util_capnp.rs"));
}

pub mod web_session_capnp {
  include!(concat!(env!("OUT_DIR"), "/web_session_capnp.rs"));
}

pub mod server;

fn main() {
    match server::main() {
        Ok(()) => {return;}
        Err(e) => {
            panic!("error: {}", e);
        }
    }
}
