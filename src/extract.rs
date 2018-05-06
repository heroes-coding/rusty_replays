use unpack::Replay;
use unpack::Hero;
use ::math::mean;
use ::math::sigma;
use ::math::exponential_smoother;

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
    pub msls: Vec<u32>
}

pub fn extract_basic_stats(filtered: Vec<[usize;2]>, n_heroes: usize) {
    let n_reps = filtered.len();
    println!("n_heroes: {}",n_heroes);
    let mut hero_stats : Vec<HeroHolder> = vec![];
    for i in 0..n_heroes{
        let mut hero = HeroHolder {
            globes: vec![],
            strucs: vec![],
            mercs: vec![],
            kda: vec![],
            mmr: vec![],
            maps: vec![],
            wins: vec![],
            msls: vec![],
            lengths: vec![],
        };
        hero_stats.push(hero);
    }

    for i in 0..n_reps {
        let  [id, team] = &filtered[i];
        let rep = &::REPLAYS.lock().unwrap()[*id];

        let won = if rep.winners == *team as u8 { 1 } else { 0 };
        let msl = rep.msl;
        let map = rep.map;
        let length = rep.length;
        for h in 0..5 {
            let hero_id = &rep.teams[*team][h];
            let hero_id = hero_id.clone() as usize;
            let hero = &rep.heroes[*team][h];
            hero_stats[hero_id].globes.push(hero.globes);
            hero_stats[hero_id].strucs.push(hero.strucs);
            hero_stats[hero_id].mercs.push(hero.mercs);
            hero_stats[hero_id].kda.push(hero.kda);
            hero_stats[hero_id].mmr.push(hero.mmr);
            hero_stats[hero_id].lengths.push(length);
            hero_stats[hero_id].msls.push(msl);
            hero_stats[hero_id].wins.push(won);
            hero_stats[hero_id].maps.push(map);
            
        }
        // println!("{}: {}",id,team);
        // let rep = &::REPLAYS.lock().unwrap()[i];
    }

    // This structure should be flat from the get go
    let mut summary_stats : Vec<f32> = vec![];
    for h in 0..n_heroes {
        let globes = &hero_stats[h].globes;
        let globes_mean = mean(globes);
        summary_stats.push(globes_mean);
        summary_stats.push(sigma(globes,globes_mean));

        let mercs = &hero_stats[h].mercs;
        let mercs_mean = mean(mercs);
        summary_stats.push(mercs_mean);
        summary_stats.push(sigma(mercs,mercs_mean));

        let kda = &hero_stats[h].kda;
        let kda_mean = mean(kda);
        summary_stats.push(kda_mean);
        summary_stats.push(sigma(kda,kda_mean));

        let mmr = &hero_stats[h].mmr;
        let mmr_mean = mean(mmr);
        summary_stats.push(mmr_mean);
        summary_stats.push(sigma(mmr,mmr_mean));

        let lengths = &hero_stats[h].lengths;
        let lengths_mean = mean(lengths);
        summary_stats.push(lengths_mean);
        summary_stats.push(sigma(lengths,lengths_mean));

    }

    println!("Samuro: {:?}, stats: {:?}, mean win rate: {}",hero_stats[0],&summary_stats[0..10],mean(&hero_stats[0].wins));
    let res = exponential_smoother(&hero_stats[0].wins, &hero_stats[0].msls);
    println!("Exponential smoother results: {:?}", res);
}
 
