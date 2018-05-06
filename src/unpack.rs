/// unpacks replays from bit packed format and stores them in their parent level lazy_statics
///
/// # Example
/// 
/// ```
/// let sample = vec![2410658863, 1595778582, 3136600576, 302937366, 1224707526, 1201005630, 2643851622, 3156422089, 2412767065, 1696752683, 2026036817, 10675041, 1958645547, 4135024757, 2987236720, 2151302744];
/// replays::unpack::parse_replays(sample,1,150);
/// // replays::print_replays((replays::REPLAYS.lock().unwrap().len()-1) as usize);
/// ```
/// 



use std::fmt;
extern crate time;
use self::time::PreciseTime;

static N_INTS : usize = 16;
static INT_LENGTHS : [usize;16] = [ 8, 5, 12, 13, 13, 9, 6, 9, 10, 8, 7, 11, 8, 5, 5, 12 ]; 
static DECODER_STARTS : [usize;16] = [0, 24, 39, 75, 114, 153, 180, 198, 225, 255, 279, 300, 333, 357, 372, 387];
static DECODERS : [u32;423] = [ 5, 1, 64, 5, 1, 63, 4, 1, 62, 6, 1, 101, 5, 1, 60, 147, 1, 0, 147, 1, 8, 60, 1, 20, 60, 1, 26, 147, 1, 7, 147, 1, 1, 100, 1, 49, 30, 1, 130, 5, 1, 96, 5, 1, 95, 100, 1, 48, 6, 1, 80, 5, 1, 79, 5, 1, 78, 5, 1, 77, 4, 1, 76, 5, 1, 75, 3, 1, 131, 6, 1, 31, 6, 1, 30, 4, 1, 90, 5, 1, 65, 5, 1, 99, 3, 1, 132, 6, 1, 87, 5, 1, 86, 5, 1, 85, 5, 1, 84, 4, 1, 83, 6, 1, 94, 5, 1, 81, 12, 1, 51, 12, 1, 50, 6, 1, 35, 6, 1, 34, 4, 1, 104, 5, 1, 103, 5, 1, 102, 5, 1, 82, 5, 1, 93, 5, 1, 92, 5, 1, 91, 6, 1, 66, 5, 1, 89, 5, 1, 88, 12, 1, 55, 147, 1, 3, 5, 1, 98, 4, 1, 97, 6, 1, 108, 5, 1, 107, 5, 1, 106, 5, 1, 105, 147, 1, 2, 12, 1, 54, 70, 1, 133, 100, 1, 44, 100, 1, 47, 100, 1, 46, 12, 1, 57, 5, 1, 119, 147, 1, 6, 12, 1, 56, 147, 1, 5, 5, 1, 121, 5, 1, 120, 6, 1, 129, 4, 1, 118, 5, 1, 117, 5, 1, 116, 6, 1, 36, 60, 1, 27, 6, 1, 122, 5, 1, 128, 5, 1, 127, 5, 1, 126, 4, 1, 125, 5, 1, 124, 5, 1, 123, 147, 1, 9, 100, 1, 45, 24, 1, 134, 60, 1, 135, 2, 1, 136, 4, 1, 137, 5, 1, 61, 5, 1, 100, 147, 1, 4, 60, 1, 29, 25, 1, 11, 25, 1, 10, 25, 1, 19, 60, 1, 28, 6, 1, 37, 12, 1, 59, 6, 1, 32, 5, 1, 72, 5, 1, 71, 6, 1, 73, 4, 1, 69, 5, 1, 68, 5, 1, 67, 6, 1, 39, 12, 1, 58, 25, 1, 13, 25, 1, 12, 6, 1, 33, 5, 1, 70, 6, 1, 38, 60, 1, 21, 25, 1, 17, 25, 1, 16, 25, 1, 15, 25, 1, 14, 321, 1, 138, 60, 1, 25, 60, 1, 24, 60, 1, 23, 60, 1, 22, 100, 1, 40, 100, 1, 43, 100, 1, 42, 100, 1, 41, 42, 1, 139, 25, 1, 18, 6, 1, 115, 5, 1, 114, 5, 1, 113, 5, 1, 112, 4, 1, 111, 5, 1, 110, 5, 1, 109, 5, 1, 74, 12, 1, 53, 12, 1, 52, 3, 1, 140 ];
const N_DATA : usize = 141;

#[derive(Debug)]
pub struct Hero {
    pub talents: [u8; 7],
    pub globes: u8,
    pub strucs: u8,
    pub mercs: u8,
    pub kda: u8,
    pub mmr: u8
}
fn build_hero(talents: [u8;7], globes: u8, strucs: u8, mercs: u8, kda: u8, mmr: u8 ) -> Hero {
    Hero { talents, globes, strucs, mercs, kda, mmr }
}

#[derive(Debug)]
pub struct Replay {
    pub heroes: [[Hero;5];2],
    pub teams: [[u8;5];2],
    pub map: u8,
    pub first_to_10: u8,
    pub first_to_20: u8,
    pub first_fort: u8,
    pub avg_lev_diff: u8,
    pub winners: u8,
    pub region: u8,
    pub build: u8,
    pub length: u8,
    pub msl: u32
}

impl fmt::Display for Replay {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "MSL: {}, Length (mins): {}, Build: {}, Winners: {}, First To 10: {}, 20: {}, Fort: {}, Avg Lev D: {}, Region: {}",self.msl, self.length, self.build, self.winners, self.first_to_10, self.first_to_20, self.first_fort, self.avg_lev_diff, self.region).expect("could not write replay");;
        for t in 0..2 {
            write!(f, "\nTeam {}:",t);
            for i in 0..5 {
                write!(f, "\n{:?}", self.heroes[t][i]).expect("could not hero data");
            }
        }
        
        write!(f, "\nTeams: {:?}", self.teams).expect("could not write teams");
        Ok(())
    }
}

fn build_replay(heroes: [[Hero;5];2], teams: [[u8;5];2], map: u8, first_to_10: u8, first_to_20: u8, first_fort: u8, avg_lev_diff: u8, winners: u8, region: u8, build: u8, length: u8, msl: u32) -> Replay {
    Replay { heroes, teams, map, first_to_10, first_to_20, first_fort, avg_lev_diff, winners, region, build, length, msl }
}

fn parse_int(data_int: u32, int_number: usize, rep_data: &mut [u8]) {
    // REVERSE THE ENDIANNESS, BABY
    let mut int = ((data_int>>24)&0xff) | // move byte 3 to byte 0
                  ((data_int<<8)&0xff0000) | // move byte 1 to byte 2
                  ((data_int>>8)&0xff00) | // move byte 2 to byte 1
                  ((data_int<<24)&0xff000000);
    // println!("Start int: {}", int);
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
    // println!("End int: {}", int);
}


pub fn parse_replays(replay_bytes: Vec<u32>, n_replays: usize, days_since_launch: u32) {
    let start = PreciseTime::now();
    let end = PreciseTime::now();
    let mut unpack_count = start.to(end);
    let mut hero_build_count = start.to(end);
    let mut replay_build_count = start.to(end);

    println!("Should be processing {} replays", n_replays);
    for r in 0..n_replays {
        let mut d: [u8; N_DATA] = [0; N_DATA];
        for i in 0..N_INTS {
            let int = replay_bytes[r*16 as usize + i];
            let start = PreciseTime::now();
            parse_int(int,i as usize, &mut d);
            let end = PreciseTime::now();
            unpack_count = unpack_count + start.to(end);
        }
        let start = PreciseTime::now();

        let teams : [[u8;5];2] = [ [d[0],d[1],d[2],d[3],d[4]], [d[5],d[6],d[7],d[8],d[9]]] ;

        let heroes : [[Hero;5];2] = [ 
            [
                build_hero([d[60], d[61], d[62], d[63], d[64], d[65], d[66]], d[20], d[30], d[50], d[10], d[40]),
                build_hero([d[67], d[68], d[69], d[70], d[71], d[72], d[73]], d[21], d[31], d[51], d[11], d[41]),
                build_hero([d[74], d[75], d[76], d[77], d[78], d[79], d[80]], d[22], d[32], d[52], d[12], d[42]),
                build_hero([d[81], d[82], d[83], d[84], d[85], d[86], d[87]], d[23], d[33], d[53], d[13], d[43]),
                build_hero([d[88], d[89], d[90], d[91], d[92], d[93], d[94]], d[24], d[34], d[54], d[14], d[44])
                ],
            [
                build_hero([d[95], d[96], d[97], d[98], d[99], d[100], d[101]], d[25], d[35], d[55], d[15], d[45]),
                build_hero([d[102], d[103], d[104], d[105], d[106], d[107], d[108]], d[26], d[36], d[56], d[16], d[46]),
                build_hero([d[109], d[110], d[111], d[112], d[113], d[114], d[115]], d[27], d[37], d[57], d[17], d[47]),
                build_hero([d[116], d[117], d[118], d[119], d[120], d[121], d[122]], d[28], d[38], d[58], d[18], d[48]),
                build_hero([d[123], d[124], d[125], d[126], d[127], d[128], d[129]], d[29], d[39], d[59], d[19], d[49])
            ]
        ];
        let end = PreciseTime::now();
        hero_build_count = hero_build_count + start.to(end);

        let start = PreciseTime::now();

        let rep = build_replay(heroes, teams, d[130], d[131], d[140], d[132], d[133], d[136], d[137]+1, d[138], d[139]+1, days_since_launch*1440 + (d[134] as u32 *60) as u32 + d[135] as u32);

        let end = PreciseTime::now();
        replay_build_count = replay_build_count + start.to(end);

        // build_replay(heroes, team0, team1, map: u8, first_to_10: u8, 
        // first_to_20: u8, first_fort: u8, avg_lev_diff: u8, winners: u8, region: u8, build: u8, msl: u32)
        if r==0 {
         println!("Replay: {}", rep);
        }
        ::add_replay(rep);
    }


    // ::print_replays(0);
    &::REPLAYS.lock().unwrap().sort_by(|a, b| a.msl.cmp(&b.msl)); // <- THIS SIMPLE LINE SORTS ALL REPLAYS LOADED BY MSL.  SWEET
    let refr = &::REPLAYS.lock().unwrap()[0];
    println!("TEAMS FROM refr:::::: {:?},hero[0]:{:?}",refr.teams,refr.heroes[0]);
    
    println!("{} seconds to unpack ints", unpack_count);
    println!("{} seconds to build heroes", hero_build_count);
    println!("{} seconds to build replays", replay_build_count);

}