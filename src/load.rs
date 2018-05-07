
use unpack::Replay;

pub fn add_replay(replay: Replay) {
    ::REPLAYS.lock().unwrap().push(replay);
}

pub fn print_replay(index: usize) {
    println!("Replay {}: {}",index,::REPLAYS.lock().unwrap()[index]);
}

pub fn add_basic_info(n_heroes: u8, franchises_and_roles: *mut u8) {
    let mut p = franchises_and_roles;
    *::N_HEROES.lock().unwrap() = n_heroes as usize;
    unsafe {
        for _i in 0..n_heroes {
            ::FRANCHISES.lock().unwrap().push(*p);
            ::ROLES.lock().unwrap().push(*p.offset(n_heroes as isize));
            p = p.offset(1);
        }
    }
    
    /*
    let roles = &mut ::ROLES.lock().unwrap().clone();
    println!("ROLES[0]: {:?}",roles[0]);

    println!("ROLES: {:?}",*::ROLES.lock().unwrap());
    println!("N_HEROES: {}",*::N_HEROES.lock().unwrap());
    assert_eq!(78,*::N_HEROES.lock().unwrap());
    */
}