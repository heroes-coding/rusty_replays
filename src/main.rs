extern crate replays;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // All of this functionality must be moved to WebAssembly called functions
    let mut franchises_and_roles: [u8;156] = [0,2,0,0,2,0,4,1,0,0,0,1,0,2,0,0,2,0,0,2,1,1,2,0,2,0,1,0,0,2,0,1,1,0,2,2,2,0,0,1,0,3,1,0,2,0,0,0,1,1,1,0,1,0,0,2,0,3,0,0,0,0,3,2,0,1,3,3,1,2,0,3,3,0,3,2,0,2,3,1,0,3,2,1,2,2,2,0,3,3,0,3,3,3,3,2,3,0,0,2,1,1,3,1,1,1,0,3,3,0,3,1,2,0,3,2,0,0,0,3,0,0,2,3,1,2,0,1,2,3,3,3,3,3,0,0,4,3,3,3,1,2,0,3,3,0,3,1,3,1,3,1,3,0,3,3];
    let p = &mut franchises_and_roles;
    replays::load::add_basic_info(78,p.as_mut_ptr());

    let filename = "ints.json";
    println!("In file {}", filename);
    let mut f = File::open(filename).expect("file not found");;
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("file not read");;
    let mut rep_ints: Vec<u32> = Vec::new();
    for line in contents.lines() {
        if line=="[" {
            println!("On first line");
        } else if line=="]" {
            println!("On last line");
        } else {
            let s: String = line.chars().filter(|x| x.is_digit(10)).collect();
            let n: u32 = s.parse().unwrap();
            rep_ints.push(n);
        }
    }
    
    let n_replays = rep_ints.len()/replays::unpack::N_INTS;
    // println!("called {}", ARRAY.lock().unwrap().len());
    // replays::add_replays(rep_ints,n_replays);
    replays::unpack::parse_replays(rep_ints,n_replays,150);

    let ateam : Vec<u8> = vec![75];
    let oteam : Vec<u8> = vec![];
    let aroles : [u8;5] = [0,0,0,0,0];
    let oroles : [u8;5] = [0,0,0,0,0];
    let maps : Vec<u8> = vec![];
    let regions : Vec<u8> = vec![];
    let modes : Vec<u8> = vec![];
    let min_msl = 0;
    let max_msl = 0;

    replays::filter::filter_replays(&ateam,&oteam, &aroles, &oroles, &maps, &regions, &modes, &min_msl, &max_msl);

    //replays::extract::extract_basic_stats(78);
    // println!("Filtered indexes: {:?}",filtered);

}