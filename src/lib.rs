mod unpack;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn open_sample_replays() {
    let sample = vec![2410658863, 1595778582, 3136600576, 302937366, 1224707526, 1201005630, 2643851622, 3156422089, 2412767065, 1696752683, 2026036817, 10675041, 1958645547, 4135024757, 2987236720, 2151302744];
    unpack::parse_replays(sample,1,150);
}