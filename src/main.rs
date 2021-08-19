#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use nb::block;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use embedded_hal::digital::v2::OutputPin;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
  // Get access to the core peripherals
  let cp = cortex_m::Peripherals::take().unwrap();
  // Get access to the device specific peripherals
  let dp = pac::Peripherals::take().unwrap();

  // Take ownership over the raw flash and rcc devices
  let mut flash = dp.FLASH.constrain();
  let mut rcc = dp.RCC.constrain();

  // Freeze the configuration of all the clocks in the system
  // and store the frozen frequencies in `clocks`
  let clocks = rcc.cfgr.freeze(&mut flash.acr);

  // Acquire the GPIOC peripheral
  let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

  // Configure gpio C pin 13 as a push-pull output.
  // The `crh` register is passed to the function in order to configure the port.
  // For pins 0-7, crl should be passed instead.
  let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
  // Configure the syst timer to trigger an update every second
  let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(1.hz());

  hprintln!("Hello, world!").unwrap();
  
  // Wait for the timer to trigger an update and change the state of the LED
  loop {
      block!(timer.wait()).unwrap();
      led.set_high().unwrap();
      block!(timer.wait()).unwrap();
      led.set_low().unwrap();
  }
}