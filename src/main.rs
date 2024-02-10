use async_std;
extern crate hidapi;
use hidapi::HidApi;


#[async_std::main]
async fn main()
{
    let api = HidApi::new().unwrap();

    let controller = api.open(1356, 2508).unwrap();

    loop {
        let mut buf = [0_u8; 256];
        match controller.read(&mut buf) {
            Ok(_data)=>{
                println!("Read:{:?}", buf);
            }
            Err(e)=>{
                eprintln!("Error:{}", e);
            }
        }
    }
}