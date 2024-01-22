use std::fmt::Debug;
use std::error::Error;

#[derive(Debug)]
pub struct Imu{
    pub micros:u64,
    pub x:f32,
    pub y:f32,
    pub z:f32,
}

impl Imu{
    pub fn new ( line:&String) -> Result<Imu, Box<dyn Error>> 
    {
		let fields:Vec<&str> = line.split(",").collect();
		Ok(Imu{
		micros : fields[1].parse::<u64>()?,
		x: fields[2].parse::<f32>()?,
		y: fields[3].parse::<f32>()?,
		z: fields[4].parse::<f32>()?,
		})
    }
}
