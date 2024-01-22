//use std::num::ErrorKind;

use serialport::{
    //SerialPortBuilder,
//		 DataBits,Parity,FlowControl,StopBits,
//		 TTYPort,
		 SerialPort};
use std::io::{self,Read,Write};
use core::time::Duration;

mod imu;

fn main() {
    let mut port = serialport::new("/dev/OFIMU", 115200u32)
	.timeout(Duration::from_millis(10))
	//.data_bits(DataBits::Eight).parity(Parity::None).stop_bits(StopBits::One).flow_control(FlowControl::None)
	.open_native().expect("Failed to open port");
    let mut buf: Vec<u8> = vec![0; 100];
    let mut line_str = String::new();
    loop{
	let avail =  port.bytes_to_read().unwrap();
	if avail == 0 {continue;}
	let av_sz:usize = avail.try_into().unwrap();
	buf.resize(av_sz,0);
	let rres = port.read_exact(buf.as_mut_slice());	
	match rres{
            Ok(_n) => {		
		let this_chunk = match std::str::from_utf8(&buf){
		    Ok(_s) => _s,
		    Err(_e) => {
			println!("to string error");
			continue;
		    },
		};
		line_str.push_str(this_chunk);		
		if  buf.last() == Some(&10u8) {  // 10=ascii \n
		    if line_str.len() > 47 && line_str.contains(','){
			
			let i = match imu::Imu::new(&line_str){
			Ok(_i) => _i,
			Err(_e) => {
			    eprintln!("{:?}", _e);
			    imu::Imu{micros:0u64,x:0f32,y:0f32,z:0f32}
			},
		    };
			
			println!("{:?}",i);	
		    }
		    line_str.clear();
		    continue;
		}
	
	    },
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
	    }
    }
}
