use async_std;
use dualshock_driver::DualShock4Driver;


#[async_std::main]
async fn main()
{
    let mut driver = DualShock4Driver::new().unwrap();

    loop {
        let ds4 = driver.read().await.unwrap();

        println!("Stick{{left_x:{}, left_y:{}, right_x:{}, right_y:{}}}", ds4.sticks.left_x, ds4.sticks.left_y, ds4.sticks.right_x, ds4.sticks.right_y);
        println!("Buttons{{circle:{}, cross:{}, triangle:{}, cube:{}, r1:{}, r2:{}, l1:{}, l2:{}}}", 
            ds4.btns.circle,
            ds4.btns.cross,
            ds4.btns.triangle,
            ds4.btns.cube,
            ds4.btns.r1,
            ds4.btns.r2,
            ds4.btns.l1,
            ds4.btns.l2);

        println!("Dpad{{up:{}, down:{}, left:{}, right:{}}}", ds4.dpad.up_key, ds4.dpad.down_key, ds4.dpad.left_key, ds4.dpad.right_key);
    }
}
