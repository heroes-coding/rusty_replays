use unpack::Replay;
use unpack::Hero;

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


pub fn filter_replays(ateam: Vec<u8>, oteam: Vec<u8>, aroles: [u8;5], oroles: [u8;5], maps: Vec<u8>, regions: Vec<u8>, modes: Vec<u8>, min_msl: u32, max_msl: u32 ) -> Vec<[usize;2]> {
    // Filters replays (already stored as an unsafe global collection)
    // 
    // The returned values are pairs of replay index, team index (0 or 1)

    let check_heroes = if ateam.len() + oteam.len() == 0 { false } else { true };
    let check_roles = if aroles.iter().fold(0, |t, n| t + n) + oroles.iter().fold(0, |t, n| t + n) == 0 { false } else { true };
    let check_map = if maps.len() == 0 { false } else { true };
    let check_region = if regions.len() == 0 { false } else { true };
    let check_mode = if modes.len() == 0 { false } else { true };
    let check_min_msl = if min_msl == 0 { false } else { true };
    let check_max_msl = if max_msl == 0 { false } else { true };
    

    println!("Check heroes: {}, check roles: {}",check_heroes,check_roles);

    let n_reps = ::REPLAYS.lock().unwrap().len();
    let mut filtered : Vec<[usize;2]> = Vec::new();

    for i in 0..n_reps {
        // get replay from parent mod (lib.rs)
        let rep = &::REPLAYS.lock().unwrap()[i];

        // construct roles for this replay on the fly instead of storing them
        let mut roles : [[u8;5];2] = [[0,0,0,0,0],[0,0,0,0,0]];
        for t in 0..2 { 
            for h in 0..5 {
                let role = ::ROLES.lock().unwrap()[rep.teams[t][h] as usize] as usize;
                roles[t][role] = roles[t][role] +1;
            }            
        }

        // loop through and check that all conditions are met for the replay by team, as certain conditions are required to match a certain side (for example, Nazeebo was first to 20).  Only need to add once
        for t in 0..2 {
            // if passed { break }; // can skip second time around if the first time passed all checks
            if check_heroes && !has_heroes(&rep.teams[t], &rep.teams[1-t], &ateam, &oteam) {
                continue 
            } else if check_roles && !enough_roles(&roles[t], &roles[1-t], &aroles, &oroles) {
                continue
            }
            let mut repAndTeam: [usize;2] = [i,t];
            filtered.push(repAndTeam);
            // let nGlobes: Vec<u8>  = rep.heroes[0].iter().map(|&Hero { talents, globes, strucs, mercs, kda, mmr }| globes).collect();
            let nGlobes: u32  = rep.heroes[0].iter().map(| ref Hero | Hero.globes as u32).sum();
            // println!("{:?}",(nGlobes as f32)/5.0);
        }

        // println!("mut roles: {:?}, passed:{}",roles,passed);
        /*
        if passed {
            let mut repAndTeam: [usize;2] = [i,team];
            filtered.push(repAndTeam);
            println!("{}: {}",i,rep);
        }
        */
    }
    filtered
    // let n_heroes = &::N_HEROES.lock().unwrap();


}