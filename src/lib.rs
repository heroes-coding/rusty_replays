#[macro_use]
extern crate lazy_static;
// extern crate rusty_machine as rm;
use std::sync::Mutex;
use std::mem;
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_void};

pub mod unpack;
pub mod filter;
pub mod extract;
pub mod load;
pub mod math;
use unpack::Replay;

lazy_static! {
    #[derive(Debug)]
    pub static ref REPLAYS: Mutex<Vec<Replay>> = Mutex::new(vec![]);
    pub static ref ROLES: Mutex<Vec<u8>> = Mutex::new(vec![]);
    pub static ref FRANCHISES: Mutex<Vec<u8>> = Mutex::new(vec![]);
    pub static ref N_HEROES: Mutex<usize> = Mutex::new(0);   
    pub static ref N_FILTERED: Mutex<usize> = Mutex::new(0);   
    pub static ref FILTERED: Mutex<Vec<[usize;2]>> = Mutex::new(vec![]);
}

#[no_mangle]
pub extern "C" fn add_basics(franchises_and_roles: *mut u8, n_heroes: u8) {
    load::add_basic_info(n_heroes,franchises_and_roles);
}


#[no_mangle]
pub extern "C" fn add_many_replays(
    data: *mut u32, n_replays_array: *mut u32, modes_array: *mut u32, 
    days_since_launch_array: *mut u32, cohorts: u32) -> u32 
{
    let mut p = data;
    let mut r = n_replays_array;
    let mut d = days_since_launch_array;
    let mut m = modes_array;
    let mut total_replays : usize = 0;
    let mut offset : usize = 0;
    unsafe {
        for _c in 0..cohorts {
            let n_replays = *r as usize;
            r = r.offset(1);
            let days_since_launch = *d;
            d = d.offset(1);
            let mode = *m as u8;
            m = m.offset(1);
            let end = offset + n_replays*unpack::N_INTS;
            let mut rep_ints : Vec<u32> = Vec::new();
            for _i in offset..end {
                rep_ints.push(*p);
                p = p.offset(1);
            }
            offset = end;
            unpack::parse_replays(rep_ints,n_replays,days_since_launch,mode);
            total_replays += n_replays;
        }
    }
    
    return total_replays as u32;
}


#[no_mangle]
pub extern "C" fn add_replays(data: *mut u32, n_replays: u32, days_since_launch: u32, mode: u32) -> u32 {
    let mut p = data;
    let n_replays = n_replays as usize;
    let mut rep_ints : Vec<u32> = Vec::new();
    for _i in 0..n_replays*unpack::N_INTS {
        unsafe {
            rep_ints.push(*p);
            p = p.offset(1);
        }
    }
    unpack::parse_replays(rep_ints,n_replays,days_since_launch, mode as u8);
    return n_replays as u32;
}

#[no_mangle]
pub extern "C" fn print_replay(replay_n: u32) -> *mut c_char {
    let replay_string = REPLAYS.lock().unwrap()[replay_n as usize].to_string();
    let s = CString::new(replay_string).unwrap();
    s.into_raw()
}

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe  {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}