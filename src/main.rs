use std::time::{Duration, Instant};
use opencv::{
    core::Vector,
    prelude::*,
    videoio,
    highgui
};
use char_device::*;
//use std::io::{Read,Write};
use std::io::prelude::*;

fn main()  {
    let mut file = CharDevice::open("/dev/ttyUSB0").expect("open error!!");
    
    let mut _count = 0u8;
    let mut _prev = Instant::now();
    //let mut line:Vec<u8> = Vec::new();
    //print!("linecap={:?} ", line.capacity() );
    //println!("line= {:?}", &line);
    
    loop {
	match  file.num_ready_bytes(){
	    Ok(_n) => {
		if _n > 0{
		    print!("avail={:?} ", _n );
		    let mut l:Vec<u8> = Vec::new();
		    l.resize(_n.try_into().unwrap(),0);
		    file.read(l.as_mut_slice() ).expect("red error");
		    println!("line={:?}",String::from_utf16(l.as_slice() as &[u16] ) );
		   //println!("{:?}", String::from_utf8(line.clone() ).unwrap() );
		}

	    },
	    Err(_e) => panic!("e = {:?}", _e),

	};
	//line.clear();
    }


}
