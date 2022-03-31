#![no_std]
#![no_main]


// Taken from https://www.youtube.com/watch?v=pj2Rk-ftcWA


pub mod gpio;

//#[allow(unused_imports)]
use efm32gg_hal::{
   gpio::GPIOExt,
   cmu::CMUExt,
  // systick::{SystickExt, SystickDelay},
   //timer::TimerExt,
 };

use panic_itm as _;

use cortex_m::{
  //asm::bkpt, 
  //iprintln, 
  //peripheral::ITM, 
  //peripheral::NVIC,
  peripheral::syst::SystClkSource, 
};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
  let corep = cortex_m::peripheral::Peripherals::take().unwrap();
  let p = efm32gg990::Peripherals::take().unwrap();

  let cmu = p.CMU.constrain().split();
  let gpios = p.GPIO.split(cmu.gpio);

  let mut syst = corep.SYST;
  syst.set_clock_source(SystClkSource::Core);
  syst.set_reload(14_000_000); 
  syst.enable_counter();

  //let mut itm = corep.ITM;
  //iprintln!(&mut itm.stim[0], "Hello World\n");
  
  let mut leds = gpio::LEDs::new(gpios.pe2, gpios.pe3);
  let mut toogle = true;

  loop {
    if toogle
    {
      toogle = false;
      leds.led0_off();
      leds.led1_on();
    } else {
      toogle = true;
      leds.led1_off();
      leds.led0_on();
    }

    while !syst.has_wrapped() {}
  }
}
