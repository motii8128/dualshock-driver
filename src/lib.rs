extern crate hidapi;
use hidapi::{HidApi, HidDevice, HidError};
pub struct JoyStick
{
    pub left_x:f32,
    pub left_y:f32,
    pub right_x:f32,
    pub right_y:f32,
}

pub struct Dpad
{
    pub up_key:bool,
    pub down_key:bool,
    pub left_key:bool,
    pub right_key:bool,   
}

pub struct Buttons
{
    pub circle:bool,
    pub cross:bool,
    pub triangle:bool,
    pub cube:bool,
    pub r1:bool,
    pub r2:bool,
    pub l1:bool,
    pub l2:bool,
    pub left_push:bool,
    pub right_push:bool
}

pub struct DualShock4
{
    device:HidDevice,
    pub sticks:JoyStick,
    pub dpad:Dpad,
    pub btns:Buttons,
}

impl DualShock4 {
    pub fn new()->Result<DualShock4, HidError>
    {
        let api = HidApi::new().unwrap();

        match api.open(1356, 2508)
        {
            Ok(dev)=>{
                let joy = JoyStick{
                    left_x:0.0,
                    left_y:0.0,
                    right_x:0.0,
                    right_y:0.0
                };
        
                let pad = Dpad{
                    up_key:false,
                    down_key:false,
                    left_key:false,
                    right_key:false
                };
        
                let btn = Buttons{
                    circle:false,
                    cross:false,
                    triangle:false,
                    cube:false,
                    r1:false,
                    r2:false,
                    l1:false,
                    l2:false,
                    left_push:false,
                    right_push:false,
                };
        
                let ds = DualShock4
                {
                    device:dev,
                    sticks:joy,
                    dpad:pad,
                    btns:btn
                };

                println!("[DualshockDriver]Open Device");
                Ok(ds)
            }
            Err(e)=>{
                println!("[DualshockDriver]:Failed to open device");
                Err(e)
            }
        }

    }
    pub async fn read(&mut self)
    {
        let mut buf = [0_u8;256];
        match self.device.read(&mut buf) {
            Ok(size)=>{
                let get_data = &buf[..size];

                let (j, btn, d) = convert(get_data);

                self.sticks = j;
                self.btns = btn;
                self.dpad = d;

            }
            Err(e)=>{
                eprintln!("{}", e);
            }
        }
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

fn convert(buf:&[u8])->(JoyStick, Buttons, Dpad)
    {
        let l_x = map(buf[1], 0.0, 255.0, -1.0, 1.0);
        let l_y = map(buf[2], 0.0, 255.0, 1.0, -1.0);
        let r_x = map(buf[3], 0.0, 255.0, -1.0, 1.0);
        let r_y = map(buf[4], 0.0, 255.0, 1.0, -1.0);

        let joy = JoyStick{left_x:l_x,left_y:l_y,right_x:r_x,right_y:r_y};
        let mut btns = Buttons{
            circle:false,
            cross:false,
            triangle:false,
            cube:false,
            r1:false,
            r2:false,
            l1:false,
            l2:false,
            left_push:false,
            right_push:false,
        };

        let mut dpad = Dpad{
            up_key:false,
            down_key:false,
            right_key:false,
            left_key:false
        };

        match buf[5] {
            0=>dpad.up_key = true,
            2=>dpad.right_key = true,
            4=>dpad.down_key = true,
            6=>dpad.left_key = true,
            24=>btns.cube = true,
            40=>btns.cross = true,
            72=>btns.circle = true,
            136=>btns.triangle = true,
            8=>(),
            _=>()
        }

        match buf[6] {
            1=>btns.l1 = true,
            2=>btns.r1 = true,
            4=>btns.l2 = true,
            8=>btns.r2 = true,
            64=>btns.left_push = true,
            128=>btns.right_push = true,
            _=>(),
        }
        (joy, btns, dpad)
    }   
