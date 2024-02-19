use serialport::SerialPort;
use std::io::{self,Read,Write};
use std::time::{Duration,Instant};
use quaternion_core::{Vector3,
		      Quaternion,
		      RotationType::*,
		      RotationSequence::XYZ,
		      to_euler_angles, sub, norm};

mod imu;

fn main() {
    let mut port = serialport::new("/dev/OFIMU", 115200u32)
	.timeout(Duration::from_millis(10))
	//.data_bits(DataBits::Eight).parity(Parity::None).stop_bits(StopBits::One).flow_control(FlowControl::None)
	.open_native().expect("Failed to open port");
    let mut count:u32 = 0;
    let mut buf: Vec<u8> = vec![0; 100];
    let mut line_str = String::new();
    let mut last = Instant::now();
    let mut q :Quaternion<f32> = (1f32,[0f32,0f32,0f32]);
    let mut ypr :Vector3<f32>  = [0f32,0f32,0f32];
    let mut ypr_last :Vector3<f32>  = [0f32,0f32,0f32];
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
			#[cfg(feature = "debug")]
			println!("to string error");
			continue;
		    },
		};
		line_str.push_str(this_chunk);		
		if  buf.last() == Some(&10u8) {  // 10=ascii \n
		    let _ = line_str.pop().unwrap();
		    if  line_str.contains(','){	
			let fields:Vec<&str> = line_str.split(",").collect();
			if fields.len() == 4{
			    q = match imu::new(&fields){
				Ok(_i) => _i,
				Err(_e) => {
				    #[cfg(feature = "debug")]
				    eprintln!("{:?}", _e);
				    (1f32,[0f32,0f32,0f32]) as Quaternion<f32>
				},
			    };
			}
			count += 1;
			ypr_last = ypr;
			ypr = to_euler_angles(Intrinsic,XYZ,q);
			let ypr1 = imu::convert_ypr(q).unwrap();
			let diff:Vector3<f32> = sub(ypr , ypr_last);
			//if diff[0].abs() >= 0.003f32{
			    //#[cfg(feature = "debug")]
			    println!("{:>7.4} {:>7.4}", ypr[0],ypr1[0]);
			//}
			//let test:f32 = q.q1*q.q2 + q.q0*q.q3;
			//if test.abs() > 0.499f32{println!("{:?}",test);}
			//println!("{:?}",q);
		    }		    
		    if last.elapsed() > Duration::from_millis(1000){
			last = Instant::now();
			//println!("{:?}", count);
			count = 0;
		    }
		    line_str.clear();
		    continue;
		}
	
	    },
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(_e) => {
		#[cfg(feature = "debug")]
		eprintln!("{:?}", _e)
	    },
	    }
    }
}
