#[macro_use]
extern crate lazy_static;
extern crate rusty_machine as rm;
use std::sync::Mutex;

pub mod unpack;
pub mod filter;
pub mod extract;
pub mod math;
use unpack::Replay;
extern crate time;
use time::PreciseTime;


lazy_static! {
    #[derive(Debug)]
    pub static ref REPLAYS: Mutex<Vec<Replay>> = Mutex::new(vec![]);
    pub static ref ROLES: Mutex<Vec<u8>> = Mutex::new(vec![]);
    pub static ref FRANCHISES: Mutex<Vec<u8>> = Mutex::new(vec![]);
    pub static ref N_HEROES: Mutex<u8> = Mutex::new(0);
    
}

fn add_replay(replay: Replay) {
    REPLAYS.lock().unwrap().push(replay);
}

pub fn print_replays(index: usize) {
    println!("Replay {}: {}",index,REPLAYS.lock().unwrap()[index]);
}

pub fn add_basic_info(n_heroes: u32, franchises_and_roles: *mut u8) {
    let mut p = franchises_and_roles;
    *N_HEROES.lock().unwrap() = n_heroes as u8;
    unsafe {
        for _i in 0..n_heroes {
            FRANCHISES.lock().unwrap().push(*p);
            ROLES.lock().unwrap().push(*p.offset(n_heroes as isize));
            p = p.offset(1);
        }
    }
    
    let roles = &mut ROLES.lock().unwrap().clone();
    println!("ROLES[0]: {:?}",roles[0]);

    println!("ROLES: {:?}",*ROLES.lock().unwrap());
    println!("N_HEROES: {}",*N_HEROES.lock().unwrap());
    assert_eq!(78,*N_HEROES.lock().unwrap());
}