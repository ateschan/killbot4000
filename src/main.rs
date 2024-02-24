use rust_gpiozero::*;
// use rust_gpiozero::PWMOutputDevice;
use k_board::{keyboard::Keyboard, keys::Keys};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("HELLO WORLD");
    let led = LED::new(19);
    let led2 = LED::new(26);
    let led3 = LED::new(20);
    let led4 = LED::new(21);
    /*
          led4.on();
          sleep(Duration::from_millis(100));
          led4.off();
          sleep(Duration::from_millis(100));
          led3.on();
          sleep(Duration::from_millis(100));
          led3.off();
          sleep(Duration::from_millis(100));
    */

    for key in Keyboard::new() {
        match key {
            Keys::Up => {
                sleep(Duration::from_millis(10));
                led.on()
            }
            Keys::Down => {
                sleep(Duration::from_millis(10));
                led2.on()
            }
            Keys::Left => {
                sleep(Duration::from_millis(10));
                led3.on()
            }
            Keys::Right => {
                sleep(Duration::from_millis(10));
                led4.on()
            }
            Keys::Enter => break,
            _ => {}
        }
        led4.off();
        led3.off();
        led2.off();
        led.off();
    }

    led2.close();
    led.close();
    led4.close();
    led3.close();
}
