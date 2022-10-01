use linux_embedded_hal::{sysfs_gpio, Delay, I2cdev};
use sysfs_gpio::{Direction, Pin};

use bno055::{BNO055OperationMode, Bno055};
use mint::Quaternion;

fn main() {
    let dev = I2cdev::new("/dev/i2c-0").unwrap();
    let mut delay = Delay {};
    let mut imu = Bno055::new(dev).with_alternative_address();
    imu.init(&mut delay)
        .expect("An error occurred while building the IMU");

    imu.set_mode(BNO055OperationMode::NDOF, &mut delay)
        .expect("An error occurred while setting the IMU mode");

    let mut quaternion: Quaternion<f32>;
    let gpio = Pin::new(24);
    let mut heartbeat: u8 = 0;
    loop {
        match imu.quaternion() {
            Ok(val) => {
                quaternion = val;
                println!("IMU Quaternion: {:?}", quaternion);
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }

        /* heartbeat */
        let _res = gpio.with_exported(|| {
            gpio.set_direction(Direction::Out)?;
            gpio.set_value(heartbeat)?;
            heartbeat = !heartbeat;
            // ...
            Ok(())
        });
    }
}
