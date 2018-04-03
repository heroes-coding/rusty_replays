#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;

mod unpack;
use unpack::Replay;
extern crate time;
use time::PreciseTime;


lazy_static! {
    #[derive(Debug)]
    static ref REPLAYS: Mutex<Vec<Replay>> = Mutex::new(vec![]);
    static ref ROLES: Mutex<Vec<u8>> = Mutex::new(vec![]);
    static ref FRANCHISES: Mutex<Vec<u8>> = Mutex::new(vec![]);
    static ref N_HEROES: Mutex<u8> = Mutex::new(0);
}

fn add_replay(replay: Replay) {
    REPLAYS.lock().unwrap().push(replay);
}


pub fn print_replays() {
    for i in 0..2 {
        println!("Replay {}: {}",i,REPLAYS.lock().unwrap()[i]);
    }
    println!("Replay length {}", REPLAYS.lock().unwrap().len());
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
    
    let mut roles = &mut ROLES.lock().unwrap().clone();
    println!("ROLES[0]: {:?}",roles[0]);

    let mut ros = &mut roles;
    println!("ROLES[0]: {:?}",ros[0]);


    println!("ROLES: {:?}",*ROLES.lock().unwrap());
    println!("N_HEROES: {}",*N_HEROES.lock().unwrap());
    assert_eq!(78,*N_HEROES.lock().unwrap());
}

pub fn add_replays(rep_ints: Vec<u32>, n_replays: usize) {
   
    /*
    let sample = vec![2410658863, 1595778582, 3136600576, 302937366, 1224707526, 1201005630, 2643851622, 3156422089, 2412767065, 1696752683, 2026036817, 10675041, 1958645547, 4135024757, 2987236720, 2151302744];
    */

    let start = PreciseTime::now();
    unpack::parse_replays(rep_ints,n_replays,150);
    let end = PreciseTime::now();
    println!("{} seconds to unpack {} replays", start.to(end),n_replays);

    /*
    let start = PreciseTime::now();
    let replays = unpack::parse_replays(rep_ints,n_replays,150);
    let end = PreciseTime::now();
    let mut reps = IntoIterator::into_iter(replays);  // Works.
    loop {
        match reps.next() {
        Some(rep) => {
            add_replay(rep);
        },
        None => break,
        }
    }
    println!("{} seconds to unpack {} replays", start.to(end),n_replays);
    */
    
}