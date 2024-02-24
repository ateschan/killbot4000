use rust_gpiozero::*;
use rust_gpiozero::PWMOutputDevice;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    
    let led = LED::new(2);
    let led2 = LED::new(3);
    let led3 = LED::new(20);
    let led4 = LED::new(21);
//    led3.set_value(1.0);
   /* loop {
      led.on();
      sleep(Duration::from_millis(100));
      led.off();
      sleep(Duration::from_millis(100));
      led2.on();
      sleep(Duration::from_millis(100));
      led2.off();
      sleep(Duration::from_millis(100));
      led3.on();
      sleep(Duration::from_millis(100));
      led3.off();
      sleep(Duration::from_millis(100));
      led3.on();
      led4.on();
      led3.on();
      sleep(Duration::from_millis(100)); 
    }
*/
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
    led4.off();
    led3.off();

    led4.close();
    led3.close();
}
