static N_INTS : usize = 16;
static INT_LENGTHS : [usize;16] = [ 8, 5, 12, 13, 13, 9, 6, 9, 10, 8, 7, 11, 8, 5, 5, 12 ]; 
static DECODER_STARTS : [usize;16] = [0, 24, 39, 75, 114, 153, 180, 198, 225, 255, 279, 300, 333, 357, 372, 387];
static DECODERS : [u32;423] = [ 5, 1, 64, 5, 1, 63, 4, 1, 62, 6, 1, 101, 5, 1, 60, 147, 1, 0, 147, 1, 8, 60, 1, 20, 60, 1, 26, 147, 1, 7, 147, 1, 1, 100, 1, 49, 30, 1, 130, 5, 1, 96, 5, 1, 95, 100, 1, 48, 6, 1, 80, 5, 1, 79, 5, 1, 78, 5, 1, 77, 4, 1, 76, 5, 1, 75, 3, 1, 131, 6, 1, 31, 6, 1, 30, 4, 1, 90, 5, 1, 65, 5, 1, 99, 3, 1, 132, 6, 1, 87, 5, 1, 86, 5, 1, 85, 5, 1, 84, 4, 1, 83, 6, 1, 94, 5, 1, 81, 12, 1, 51, 12, 1, 50, 6, 1, 35, 6, 1, 34, 4, 1, 104, 5, 1, 103, 5, 1, 102, 5, 1, 82, 5, 1, 93, 5, 1, 92, 5, 1, 91, 6, 1, 66, 5, 1, 89, 5, 1, 88, 12, 1, 55, 147, 1, 3, 5, 1, 98, 4, 1, 97, 6, 1, 108, 5, 1, 107, 5, 1, 106, 5, 1, 105, 147, 1, 2, 12, 1, 54, 70, 1, 133, 100, 1, 44, 100, 1, 47, 100, 1, 46, 12, 1, 57, 5, 1, 119, 147, 1, 6, 12, 1, 56, 147, 1, 5, 5, 1, 121, 5, 1, 120, 6, 1, 129, 4, 1, 118, 5, 1, 117, 5, 1, 116, 6, 1, 36, 60, 1, 27, 6, 1, 122, 5, 1, 128, 5, 1, 127, 5, 1, 126, 4, 1, 125, 5, 1, 124, 5, 1, 123, 147, 1, 9, 100, 1, 45, 24, 1, 134, 60, 1, 135, 2, 1, 136, 4, 1, 137, 5, 1, 61, 5, 1, 100, 147, 1, 4, 60, 1, 29, 25, 1, 11, 25, 1, 10, 25, 1, 19, 60, 1, 28, 6, 1, 37, 12, 1, 59, 6, 1, 32, 5, 1, 72, 5, 1, 71, 6, 1, 73, 4, 1, 69, 5, 1, 68, 5, 1, 67, 6, 1, 39, 12, 1, 58, 25, 1, 13, 25, 1, 12, 6, 1, 33, 5, 1, 70, 6, 1, 38, 60, 1, 21, 25, 1, 17, 25, 1, 16, 25, 1, 15, 25, 1, 14, 321, 1, 138, 60, 1, 25, 60, 1, 24, 60, 1, 23, 60, 1, 22, 100, 1, 40, 100, 1, 43, 100, 1, 42, 100, 1, 41, 42, 1, 139, 25, 1, 18, 6, 1, 115, 5, 1, 114, 5, 1, 113, 5, 1, 112, 4, 1, 111, 5, 1, 110, 5, 1, 109, 5, 1, 74, 12, 1, 53, 12, 1, 52, 3, 1, 140 ];
const N_DATA : usize = 141;

#[derive(Debug)]
struct Hero {
    hero: u8,
    talents: [u8; 7],
    globes: u8,
    strucs: u8,
    mercs: u8,
    kda: u8,
    mmr: u8
}
fn build_hero(hero: u8, talents: [u8;7], globes: u8, strucs: u8, mercs: u8, kda: u8, mmr: u8 ) -> Hero {
    Hero { hero, talents, globes, strucs, mercs, kda, mmr }
}


#[derive(Debug)]
struct Replay {
    heroes: Vec<Hero>,
    map: u8,
    first_to_10: u8,
    first_to_20: u8,
    first_fort: u8,
    avg_lev_diff: u8,
    winners: u8,
    region: u8,
    build: u8,
    length: u8,
    msl: u32
}
fn build_replay(heroes: Vec<Hero>, map: u8, first_to_10: u8, first_to_20: u8, first_fort: u8, avg_lev_diff: u8, winners: u8, region: u8, build: u8, length: u8, msl: u32) -> Replay {
    Replay { heroes, map, first_to_10, first_to_20, first_fort, avg_lev_diff, winners, region, build, length, msl }
}

fn parse_int(data_int: u32, int_number: usize, rep_data: &mut [u8]) {
    // REVERSE THE ENDIANNESS, BABY
    let mut int = ((data_int>>24)&0xff) | // move byte 3 to byte 0
                  ((data_int<<8)&0xff0000) | // move byte 1 to byte 2
                  ((data_int>>8)&0xff00) | // move byte 2 to byte 1
                  ((data_int<<24)&0xff000000);
    println!("{}", int);
    let n_pieces = INT_LENGTHS[int_number];
    let decoder_offset = DECODER_STARTS[int_number];
    for i in 0..n_pieces {
        let offset = decoder_offset + i*3 as usize;
        let max = DECODERS[offset];
        let mult = DECODERS[offset + 1];
        let out = DECODERS[offset + 2] as usize;
        let value = int % max * mult;
        int = int / max;
        rep_data[out] = value as u8;
    }
}

pub fn parse_replays(replay_bytes: Vec<u32>, n_replays: usize, days_since_launch: u32) {
    for r in 0..n_replays {
        let mut d: [u8; N_DATA] = [0; N_DATA];
        for i in 0..N_INTS {
            let int = replay_bytes[r*16 as usize + i];
            parse_int(int,i as usize, &mut d);
        }
        let mut heroes: Vec<Hero> = Vec::new();
        for h in 0..10 {
            let mut tals : [u8; 7] = [0; 7];
            for t in 0..7 {
                let index = 60+h*7+t;
                tals[t] = d[index];
            }
            let hero = build_hero( d[h], tals, d[20+h], d[30+h], d[50+h], d[10+h], d[40+h]);
            heroes.push(hero);
        }
        let rep = build_replay(heroes, d[130], d[131], d[140], d[132], d[133], d[136], d[137]+1, d[138], d[139]+1, days_since_launch*1440 + (d[134]*60) as u32 + d[135] as u32);
        // build_replay(heroes, map: u8, first_to_10: u8, 
        // first_to_20: u8, first_fort: u8, avg_lev_diff: u8, winners: u8, region: u8, build: u8, msl: u32)
        println!("Replay: {:?},", rep);
    }
}