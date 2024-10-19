use rust_gpiozero::*;
// use rust_gpiozero::PWMOutputDevice;
use k_board::{keyboard::Keyboard, keys::Keys};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("HELLO WORLD");
    let mut led = LED::new(19);
    let mut led2 = LED::new(26);
    let mut led3 = LED::new(20);
    let mut led4 = LED::new(21);
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
                //right wheel bkwrd
                Keys::Up => {
                    led.toggle();
                }
                //left wheel bkwrd
                Keys::Down => {
                    led2.toggle();
                }
                //left wheel fwd 
                Keys::Left => {
                    led3.toggle();

                }
                //right wheel fwd 
                Keys::Right => {
                    led4.toggle();
                }
                Keys::Enter => break,
                _ => {}
                
            }
        }
/*
    led.on();
    led2.on();
    sleep(Duration::from_millis(4000));
*/
    led3.off();
    led3.close();
    led4.off();
    led4.close();
    led.off();
    led2.off();
    led.close();
    led2.close();
}
