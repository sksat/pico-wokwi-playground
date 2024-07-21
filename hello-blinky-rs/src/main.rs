#![no_main]
#![no_std]

// Hardware Abstraction Layer
use rp_pico::hal;
// periheral access crate
use hal::pac;

use panic_halt as _;

// traits
use core::fmt::Write;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::StatefulOutputPin;
use hal::clocks::Clock;
use hal::fugit::RateExtU32;

use hal::uart::*;

use defmt_rtt as _;

#[hal::entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .unwrap();

    let mut timer = hal::timer::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led_pin = pins.gpio5.into_push_pull_output();

    let uart_pins = (
        // UART TX (characters sent from RP2040) on pin 1 (GPIO0)
        pins.gpio0.into_function(),
        // UART RX (characters received by RP2040) on pin 2 (GPIO1)
        pins.gpio1.into_function(),
    );
    let mut uart = hal::uart::UartPeripheral::new(pac.UART0, uart_pins, &mut pac.RESETS)
        .enable(
            UartConfig::new(9600.Hz(), DataBits::Eight, None, StopBits::One),
            clocks.peripheral_clock.freq(),
        )
        .unwrap();

    writeln!(uart, "Hello, world!\r").unwrap();

    let mut value = 0u32;
    loop {
        led_pin.toggle().unwrap();
        writeln!(uart, "value: {value:02}\r").unwrap();
        defmt::println!("Hello from defmt");
        timer.delay_ms(500);
        value += 1
    }
}
