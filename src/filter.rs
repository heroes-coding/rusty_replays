use unpack::Replay;

pub fn has_heroes(team0: &[u8;5], team1: &[u8;5], ateam: &Vec<u8>, oteam: &Vec<u8>) -> bool {
    // checks if both teams (switched to check both sides) fulfill the hero conditions for both teams
    let n_a = ateam.len();
    let n_o = oteam.len();
    for i in 0..n_a {
        if !team0.contains(&ateam[i]) {
            return false
        }
    }
    for i in 0..n_o {
        if !team1.contains(&oteam[i]) {
            return false
        }
    }
    true
}

pub fn enough_roles(roles0: &[u8;5], roles1: &[u8;5], aroles: &[u8;5], oroles: &[u8;5]) -> bool {
    for i in 0..5 {
        if roles0[i] < aroles[i] {
            return false
        } else if roles1[i] < oroles[i] {
            return false
        }
    }
    true
}


pub fn filter_replays(ateam: Vec<u8>, oteam: Vec<u8>, aroles: [u8;5], oroles: [u8;5], maps: Vec<u8>, regions: Vec<u8>, modes: Vec<u8>, min_msl: u32, max_msl: u32 ) {
    let check_heroes = if ateam.len() + oteam.len() == 0 { false } else { true };
    let check_roles = if aroles.iter().fold(0, |t, n| t + n) + oroles.iter().fold(0, |t, n| t + n) == 0 { false } else { true };
    let check_map = if maps.len() == 0 { false } else { true };
    let check_region = if regions.len() == 0 { false } else { true };
    let check_mode = if modes.len() == 0 { false } else { true };
    let check_min_msl = if min_msl == 0 { false } else { true };
    let check_max_msl = if max_msl == 0 { false } else { true };


    println!("Check heroes: {}, check roles: {}",check_heroes,check_roles);

    let n_reps = ::REPLAYS.lock().unwrap().len();
    let mut filtered : Vec<usize> = Vec::new();
    let mut count = 0;
    for i in 0..n_reps {
        let rep = &::REPLAYS.lock().unwrap()[i];
        if check_heroes && !has_heroes(&rep.team0, &rep.team1, &ateam, &oteam) && !has_heroes(&rep.team1, &rep.team0, &ateam, &oteam) {
            continue
        } else if check_roles && !enough_roles(&rep.roles0, &rep.roles1, &aroles, &oroles) && !enough_roles(&rep.roles1, &rep.roles0, &aroles, &oroles) {
            continue
        }

        filtered.push(i);
        println!("{}",rep);
    }

    println!("Filtered indexes: {:?}",filtered);
}