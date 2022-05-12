use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref PEERS_AGE: Mutex<HashMap<u16, u16>> = Mutex::new(HashMap::new());
}

pub fn api_addr_from_cluster_addr(_cluster_addr: String) -> String {
    String::new()
}

pub fn status_from_age(id: u16, age: u16) -> u32 {
    let mut peers = PEERS_AGE.lock().unwrap();
    match peers.insert(id, age) {
        Some(old_age) => {
            if old_age < age {
                1
            } else {
                0
            }
        }
        None => 3,
    }
}

pub fn inc<'a>(n: &'a mut u16) -> u16 {
    let r = n.clone();
    *n = *n + 1;
    return r;
}
