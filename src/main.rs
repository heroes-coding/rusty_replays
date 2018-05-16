extern crate replays;
use std::fs::File;
use std::io::prelude::*;
use replays::math::upper_month_from_msl;
use replays::math::date_from_msl;


fn read_ints(filename: String) -> Vec<u32> {
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
    rep_ints
}

fn main() {
    // All of this functionality must be moved to WebAssembly called functions
    let mut franchises_and_roles: [u8;156] = [0,2,0,0,2,0,4,1,0,0,0,1,0,2,0,0,2,0,0,2,1,1,2,0,2,0,1,0,0,2,0,1,1,0,2,2,2,0,0,1,0,3,1,0,2,0,0,0,1,1,1,0,1,0,0,2,0,3,0,0,0,0,3,2,0,1,3,3,1,2,0,3,3,0,3,2,0,2,3,1,0,3,2,1,2,2,2,0,3,3,0,3,3,3,3,2,3,0,0,2,1,1,3,1,1,1,0,3,3,0,3,1,2,0,3,2,0,0,0,3,0,0,2,3,1,2,0,1,2,3,3,3,3,3,0,0,4,3,3,3,1,2,0,3,3,0,3,1,3,1,3,1,3,0,3,3];
    let p = &mut franchises_and_roles;
    replays::load::add_basic_info(78,p.as_mut_ptr());
    let rep_ints = read_ints("ints.json".to_string());
    
    let n_replays = rep_ints.len()/replays::unpack::N_INTS;
    // println!("called {}", ARRAY.lock().unwrap().len());
    // replays::add_replays(rep_ints,n_replays);
    replays::unpack::parse_replays(rep_ints,n_replays,9000,1);

    let ateam : Vec<u8> = vec![];
    let oteam : Vec<u8> = vec![3];
    let aroles : [u8;5] = [0,0,0,0,0];
    let oroles : [u8;5] = [0,0,0,0,0];
    let maps : Vec<u8> = vec![];
    let regions : Vec<u8> = vec![1,2,3,5];
    let modes : Vec<u8> = vec![];
    let min_msl = 0;
    let max_msl = 0;

    let n_base = replays::filter::filter_replays(&ateam,&oteam, &aroles, &oroles, &maps, &regions, &modes, &min_msl, &max_msl);

    replays::extract::extract_basic_stats(ateam, 0.);
    println!("Base filtered and returned {} replays",n_base);
    let ptr = replays::get_filtered_msl();
    println!("Got this many filtered? {}", unsafe {*ptr});


    let xs = read_ints("MSLs.json".to_string());
    let minutes = 0;
    let n_points = xs.len();
    let mut y_counts : Vec<f32> = vec![];
    let mut upper_x = xs[n_points-1];
    let mut upper_month = upper_month_from_msl(upper_x);
    println!("First upper_month: {:?}", upper_month);
    let mut y_count : f32 = 0.;
    println!("Min x: {} ({:?}), max x: {} ({:?})",xs[0],date_from_msl(xs[0]),xs[n_points-1],date_from_msl(xs[n_points-1]));
    for p in 0..n_points {
        let point = n_points-p-1;
        let x = xs[point];
        let condition = if minutes > 0 { x < upper_x - minutes } else { date_from_msl(x) < upper_month };
        if condition {
            y_counts.push(y_count);
            y_count = 0.;
            if minutes > 0 { 
                upper_x = upper_x - minutes 
            }
            else {
                println!("upper_month before {:?}:   {:?}", date_from_msl(x), upper_month);
                upper_month = upper_month_from_msl(x);
                println!("upper_month after {:?}:   {:?}", date_from_msl(x), upper_month);
            }
        }
        y_count += 1.;
        if y_counts.len() > 3 { break }
    }
    y_counts.push(y_count);
    println!("y_counts: {:?}",y_counts);

}