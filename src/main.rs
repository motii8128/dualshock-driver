use async_std;
use dualshock_driver::DualShock4;


#[async_std::main]
async fn main()
{
    let mut ds = DualShock4::new();

    loop {
        let _ = ds.read();
        println!("Stick:{{left_x:{}, left_y:{}, right_x:{}, right_y:{}}}", ds.sticks.left_x, ds.sticks.left_y, ds.sticks.right_x, ds.sticks.right_y);
    }
}