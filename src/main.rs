use std::time::{Duration, Instant};
use opencv::{
    core::Vector,
    prelude::*,
    videoio,
    highgui
};
use char_device::*;
use std::io::{Read,Write};

fn main()  {
    let mut file = CharDevice::open("/dev/ttyUSB0").expect("open error!!");
    
    let mut _count = 0u8;
    let mut _prev = Instant::now();
    loop {
	println!("line");
    }


}
