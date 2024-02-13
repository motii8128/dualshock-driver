use async_std;
use dualshock_driver::DualShock4;


#[async_std::main]
async fn main()
{
    let mut ds = DualShock4::new().unwrap();

    loop {
        let _ = ds.read();
        println!("Stick:{{left_x:{}, left_y:{}, right_x:{}, right_y:{}}}", ds.sticks.left_x, ds.sticks.left_y, ds.sticks.right_x, ds.sticks.right_y);
        println!("Buttons:{{circle:{}, cross:{}, triangle:{}, cube:{}, r1:{}, r2:{}, l1:{}, l2:{}}}", ds.btns.circle, ds.btns.cross, ds.btns.triangle, ds.btns.cube, ds.btns.r1, ds.btns.r2, ds.btns.l1, ds.btns.l2);
        println!("push_left:{}, push_right:{}", ds.btns.left_push, ds.btns.right_push);
        println!("Dpad:{{up:{}, down:{}, right:{}, left:{}}}", ds.dpad.up_key, ds.dpad.down_key, ds.dpad.right_key, ds.dpad.left_key);
    }
}