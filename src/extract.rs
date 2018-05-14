use ::math::mean;
use ::math::sigma;
use ::math::exponential_smoother;
use unpack::Hero;

#[derive(Debug)]
pub struct HeroHolder {
    pub globes: Vec<u8>,
    pub strucs: Vec<u8>,
    pub mercs: Vec<u8>,
    pub kda: Vec<u8>,
    pub mmr: Vec<u8>,
    pub maps: Vec<u8>,
    pub wins: Vec<u8>,
    pub lengths: Vec<u8>,
    pub msls: Vec<u32>,
    pub first_10_results: Vec<u8>,
    pub first_20_results: Vec<u8>,
    pub first_fort_results: Vec<u8>,
    pub avg_lev_diff: Vec<u8>
}

pub fn initialize_hero_stats_holder() -> HeroHolder {
    HeroHolder {
        globes: vec![],
        strucs: vec![],
        mercs: vec![],
        kda: vec![],
        mmr: vec![],
        maps: vec![],
        wins: vec![],
        msls: vec![],
        lengths: vec![],
        first_10_results: vec![],
        first_20_results: vec![],
        first_fort_results: vec![],
        avg_lev_diff: vec![],
    }
}

pub fn add_stats(stats_holder: &mut HeroHolder, hero: &Hero, 
    msl: &u32, map: &u8, length: &u8, won: &u8, 
    first_to_10: &bool, first_to_20: &bool, first_fort: &bool, avg_lev_diff: &u8) 
{
    stats_holder.globes.push(hero.globes);
    stats_holder.strucs.push(hero.strucs);
    stats_holder.mercs.push(hero.mercs);
    stats_holder.kda.push(hero.kda);
    stats_holder.mmr.push(hero.mmr);
    stats_holder.lengths.push(*length);
    stats_holder.msls.push(*msl);
    stats_holder.wins.push(*won);
    stats_holder.maps.push(*map);
    stats_holder.avg_lev_diff.push(*avg_lev_diff);
    if *first_to_10 { stats_holder.first_10_results.push(*won) };
    if *first_to_20 { stats_holder.first_20_results.push(*won) };
    if *first_fort { stats_holder.first_fort_results.push(*won) };
}

pub fn pack_stats(summary_stats: &mut Vec<f32>, stats_holder: &HeroHolder) {
    summary_stats.push(stats_holder.wins.len() as f32);
    summary_stats.push(mean(&stats_holder.wins));
    summary_stats.push(mean(&stats_holder.first_10_results));
    summary_stats.push(mean(&stats_holder.first_20_results));
    summary_stats.push(mean(&stats_holder.first_fort_results));

    let avg_lev_diffs = &stats_holder.avg_lev_diff;
    let avg_lev_diff_mean = mean(&avg_lev_diffs);
    summary_stats.push((avg_lev_diff_mean - 35.)/10.);
    summary_stats.push(sigma(&avg_lev_diffs,avg_lev_diff_mean)/10.);

    let strucs = &stats_holder.strucs;
    let strucs_mean = mean(&strucs);
    summary_stats.push(strucs_mean);
    summary_stats.push(sigma(&strucs,strucs_mean));

    let globes = &stats_holder.globes;
    let globes_mean = mean(&globes);
    summary_stats.push(globes_mean);
    summary_stats.push(sigma(&globes,globes_mean));

    let mercs = &stats_holder.mercs;
    let mercs_mean = mean(&mercs);
    summary_stats.push(mercs_mean);
    summary_stats.push(sigma(&mercs,mercs_mean));

    let kda = &stats_holder.kda;
    let kda_mean = mean(&kda);
    summary_stats.push(kda_mean);
    summary_stats.push(sigma(&kda,kda_mean));

    let mmr = &stats_holder.mmr;
    let mmr_mean = mean(&mmr);
    summary_stats.push(mmr_mean);
    summary_stats.push(sigma(&mmr,mmr_mean));

    let lengths = &stats_holder.lengths;
    let lengths_mean = mean(&lengths);
    summary_stats.push(lengths_mean);
    summary_stats.push(sigma(&lengths,lengths_mean));
}

pub fn add_timed_data(summary_stats: &mut Vec<f32>, stats_holder: &HeroHolder) {
    let res = exponential_smoother(&stats_holder.wins, &stats_holder.msls, 100.);
    let n_points = res[0].len();
    summary_stats.push((n_points*2) as f32);
    for d in 0..2 {
        for p in 0..n_points {
            summary_stats.push(res[d][p]);
        }
    }
}

pub fn extract_basic_stats() -> *mut f32 {
    let n_reps = *::N_FILTERED.lock().expect("Could not open the N_FILTERED mutex");
    let n_heroes = *::N_HEROES.lock().expect("Could not open the N_HEROES mutex");
    println!("n_reps: {}, n_heroes: {}",n_reps,n_heroes);
    let mut overall = initialize_hero_stats_holder();
    let mut hero_stats : Vec<HeroHolder> = vec![];
    for _ in 0..n_heroes{
        let mut hero = initialize_hero_stats_holder();
        hero_stats.push(hero);
    }

    for i in 0..n_reps {
        let  [id, team] = &::FILTERED.lock().unwrap()[i];
        let rep = &::REPLAYS.lock().unwrap()[*id];
        
        let won = if rep.winners == *team as u8 { 1 } else { 0 };
        let msl = rep.msl;
        let map = rep.map;
        let length = rep.length;
        let first_to_10 = rep.first_to_10 == *team as u8;
        let first_to_20 = rep.first_to_20 == *team as u8;
        let first_fort = rep.first_fort == *team as u8;
        let avg_lev_diff = if *team == 1 { rep.avg_lev_diff } else { 70-rep.avg_lev_diff };
        for h in 0..5 {
            let hero_id = &rep.teams[*team][h];
            let hero_id = hero_id.clone() as usize;
            let hero = &rep.heroes[*team][h];
            add_stats(&mut hero_stats[hero_id],hero,&msl,&map,&length, &won, &first_to_10, &first_to_20, &first_fort, &avg_lev_diff);
            add_stats(&mut overall,hero,&msl,&map,&length, &won, &first_to_10, &first_to_20, &first_fort, &avg_lev_diff);
        }
        // println!("{}: {}",id,team);
        // let rep = &::REPLAYS.lock().unwrap()[i];
    }

    // This structure should be flat from the get go
    let mut summary_stats : Vec<f32> = vec![];

    // Add timed data for overall stats
    add_timed_data(&mut summary_stats, &overall);


    let n_stats = 19;
    for h in 0..n_heroes {
        pack_stats(&mut summary_stats, &hero_stats[h]);
        let start = 0+n_stats*h;
        let end = (1+h)*n_stats;
        // println!("Hero {} (start: {}, end: {}) stats: {:?}",h,start,end,&summary_stats[start..end]);
    }
    pack_stats(&mut summary_stats, &overall);
    // println!("Overall stats: {:?}",&summary_stats[n_stats*n_heroes..(1+n_heroes)*n_stats]);
    // summary_stats
    let mut summary_mutex = ::RESULTS.lock().expect("Could not lock RESULTS mutex");
    *summary_mutex = summary_stats;

    unsafe {
        summary_mutex.as_mut_ptr()
    }
    
    // let res = exponential_smoother(&hero_stats[0].wins, &hero_stats[0].msls);
    // println!("Exponential smoother results: {:?}", res);
}
 
