use std::time::{Duration, Instant};
use opencv::{
    core::Vector,
    prelude::*,
    videoio,
    highgui
}; // Note, the namespace of OpenCV is changed (to better or worse). It is no longer one enormous.
fn main() -> opencv::Result<(), opencv::Error> {
    // Open a GUI window
    //highgui::named_window("window", highgui::WINDOW_AUTOSIZE)?;
    // Open the web-camera (assuming you have one)
    let _params :Vector<i32> = Vector::from_slice(&[videoio::CAP_PROP_FPS,60,videoio::CAP_PROP_FRAME_WIDTH,640,videoio::CAP_PROP_FRAME_HEIGHT,480]);
    //let mut cam = videoio::VideoCapture::new_with_params(0, videoio::CAP_V4L2,&_params).expect("cant capture");
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_V4L).expect("cant capture");
    let mut frame = Mat::default(); // This array will store the web-cam data
    // Read the camera
    // and display in the window
    let mut prev = Instant::now();
    loop {
        match cam.read(&mut frame){
	    Ok()=> println!("{:?}fps", 1000f32/prev.elapsed().as_millis() as f32),
	}
        //highgui::imshow("window", &frame)?;
	 prev = Instant::now();

    }
    Ok(())
}
