use std::{thread, time};
use char_device::*;
use std::io::Write;

fn write_report(report:Vec<u8>){
    let mut file = CharDevice::open("/dev/hidg0").expect("open error!!");
        file.write(report.as_slice()).expect("write error!!"); 
}

fn main(){
let millis = time::Duration::from_millis(250);
   for x in 1..101{
   let s1= vec![0x00,0x7f,0x00];
   let s2= vec![0x00,0x90,0x00];
   write_report(s1);
   thread::sleep(millis);
   write_report(s2);
   thread::sleep(millis);
   
   }
}