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
    let mut line:Vec<u8> = String::from("line").into_bytes();
     println!("line= {:?}", &line);
    loop {
	match  file.num_ready_bytes(){
	    Ok(_n) => {
		if _n > 0{
		    line.reserve(_n.try_into().unwrap() );
		    println!("{:?} {:?}", _n, line.capacity() );
		   //let _o = file.read_to_end(&mut line).unwrap();
		   //println!("{:?}",_o);
		    //println!("{:?}", String::from_utf8(line.clone() ).unwrap() );
		}

	    },
	    Err(_e) => panic!("e = {:?}", _e),

	};
	//line.clear();
    }


}
