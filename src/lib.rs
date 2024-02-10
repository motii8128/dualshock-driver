extern crate hidapi;
use hidapi::HidApi;

pub struct DualShock4
{
    api:HidApi,
    pub left_x:f32,
    pub left_y:f32,
    pub right_x:f32,
    pub right_y:f32,
    pub up_key:bool,
    pub down_key:bool,
    pub left_key:bool,
    pub right_key:bool,
}

impl DualShock4 {
    pub fn new()
    {
        let api = HidApi::new().unwrap();

        let dev = api.open(1356, 2508).unwrap();
    }
    pub fn get_data(buf:&[u8])
    {
        let l_x = map(buf[1], 0.0, 255.0, -1.0, 1.0);
        let l_y = map(buf[2], 0.0, 255.0, -1.0, 1.0);
        let r_x = map(buf[3], 0.0, 255.0, -1.0, 1.0);
        let r_y = map(buf[4], 0.0, 255.0, -1.0, 1.0);

    }   
}

fn map(value:u8,in_min:f32, in_max:f32, out_min:f32, out_max:f32)->f32
{
    let mut result = (value as f32 - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;

    if result.abs() < 0.07
    {
        result = 0.0;
    }

    result
}