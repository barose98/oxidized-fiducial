#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;

use ambiq_hal as hal;
use hal::prelude::*;
use embedded_hal::spi;
use ufmt::uwrite;
use ufmt::uwriteln;

use bit_byte_structs::bus::{Interface, InterfaceError};

extern crate icm20948;
use icm20948::ICMCommon;
/*
#[panic_handler]
fn p(&PanicInfo) -> ! {

}
*/
#[entry]
fn main() -> ! {
    let mut dp =  hal::pac::Peripherals::take().unwrap();
    let core =  hal::pac::CorePeripherals::take().unwrap();

    let mut delay = hal::delay::Delay::new(core.SYST, &mut dp.CLKGEN);

    let pins = hal::gpio::Pins::new(dp.GPIO);
    let mut led = pins.d19.into_push_pull_output();
    let mut icm_pwr = pins.d18.into_push_pull_output();
    let mut icm_cs = pins.icmcs.into_push_pull_output();

    // set up serial
    let mut serial = hal::uart::Uart0::new(dp.UART0, pins.tx0, pins.rx0);
    uwriteln!(serial, "\rSetting up SPI..").unwrap() ;
    let mut spi = hal::spi::Spi0::new(
        dp.IOM0,
        pins.d12,
        pins.d13,
        pins.d11,
        hal::spi::Freq::F400kHz,
        spi::MODE_0,
    );
    uwriteln!(serial, "\rSetting up ICM..").unwrap();
    icm_pwr.set_high().unwrap();
    let mut icm = icm20948::ICMSPI::new(&mut spi, &mut icm_cs).unwrap();
    match icm.init(&mut spi, &mut icm_cs, &mut delay){
	    Ok(_i) => uwriteln!(serial, "\rInitialized SUCCESS").unwrap(),
	    Err(_e) => {
		uwriteln!(serial, "\rInitialized ERROR ").unwrap();
		panic!();
	    },
	};
    //let mut icm_com = ICMCommon::new().unwrap();
    //icm_com.init(&mut spi, &mut delay).unwrap();
    
    // Set up RTC
    let mut rtc = hal::rtc::Rtc::new(dp.RTC, &mut dp.CLKGEN);
    rtc.enable();
    let mut timestamp = rtc.now().timestamp_millis();
    loop {
        led.toggle().unwrap();
        delay.delay_ms(100u32);
        let now = rtc.now().timestamp_millis();
	uwrite!(serial, "\rloop ").unwrap();
	
	/*
let (xa, ya, za, xg, yg, zg) =   match icm.get_values_accel_gyro(&mut spi, &mut icm_cs){
	    Ok(_ag) => {
		uwriteln!(serial, "SUCCESS").unwrap();
		_ag
	    },
	    Err(_e) => {
		uwriteln!(serial, "ERROR ").unwrap();
		panic!();
	    },
	};
		*/
	//let (xa, ya, za, xg, yg, zg) =   icm.scale_raw_accel_gyro(icm.get_values_accel_gyro(&mut spi, &mut icm_cs).unwrap());
	//uwriteln!(serial, "\r{}, {}, {}, {}, {}, {}", xa, ya, za, xg, yg, zg).unwrap() ;
	uwriteln!(serial, "\r{}", now).unwrap();
        timestamp = now;
    }
}

