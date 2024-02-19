//use std::ops::Sub;
//use std::fmt::{Debug,Formatter};
use std::error::Error;
use quaternion_core::{Vector3, Quaternion, normalize};

#[derive(Copy,Clone)]
pub struct Ypr{
 pub yaw:f32,
 pub pitch:f32,
 pub roll:f32,
}
#[derive(Copy,Clone)]
pub struct Quat{
    pub q0:f32,
    pub q1:f32,
    pub q2:f32,
    pub q3:f32,
}
pub fn new ( fields:&Vec<&str> ) -> Result<Quaternion<f32>, Box<dyn Error>> 
{
	let q0:f32 = fields[0].parse::<f32>()?;
	let q1:f32 = fields[1].parse::<f32>()?;
	let q2:f32 = fields[2].parse::<f32>()?;
	let q3:f32 = fields[3].parse::<f32>()?;
	Ok(normalize( (q0,[q1,q2,q3]) as Quaternion<f32>))
		
}
pub fn convert_ypr(q :Quaternion<f32>) -> Result<Vector3<f32>,Box<dyn Error>>
{
    let q0 :f32 = q.0;
    let q1 :f32 = q.1[0];
    let q2 :f32 = q.1[1];
    let q3 :f32 = q.1[2];
	let y1 :f32 = 2f32 * q0*q2 - 2f32 * q1*q3;
	let y2 :f32 = 1f32 - q2*q2 - q3*q3 ;
	let y :f32 =  (y1 as f64).atan2(y2 as f64).to_degrees() as f32;
	
	let p :f32 = ((2f32 * q0*q3 + 2f32 * q1*q2) as f64).asin().to_degrees() as f32;
	
	let r1 :f32 = 2f32 * q1*q3 - 2f32 * q2*q3;
	let r2 :f32 = 1f32 - 2f32 * q3*q3  - 2f32 * q1*q1 ;
	let r :f32 = (r1 as f64).atan2(r2 as f64).to_degrees() as f32;

	Ok([y,p,r] as Vector3<f32>)
}

/*

impl Ypr{
    pub fn diff(&self, other :&Ypr) -> Result<Ypr, Box<dyn Error>>
    {
	Ok(Ypr{
	    yaw:(self.yaw - other.yaw),
	    pitch:(self.pitch - other.pitch),
	    roll:(self.roll - other.roll),
	})
    }
}

impl Debug for Ypr{
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
	write!(f,"{1:.0$} {2:.0$} {3:.0$}",3,self.yaw,self.pitch,self.roll)
    }
}
impl Debug for Quat{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
	write!(f,"{1:.0$} {2:.0$} {3:.0$} {4:.0$} ",3,self.q0,self.q1,self.q2,self.q3)
    }

}


impl Ypr{
    pub fn diff(&self, other :&Ypr) -> Result<Ypr, Box<dyn Error>>
    {
	Ok(Ypr{
	    yaw:(self.yaw - other.yaw),
	    pitch:(self.pitch - other.pitch),
	    roll:(self.roll - other.roll),
	})
    }
}

impl Debug for Ypr{
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
	write!(f,"{1:.0$} {2:.0$} {3:.0$}",3,self.yaw,self.pitch,self.roll)
    }
}
impl Debug for Quat{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
	write!(f,"{1:.0$} {2:.0$} {3:.0$} {4:.0$} ",3,self.q0,self.q1,self.q2,self.q3)
    }

}
*/
