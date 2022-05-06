use example_api::*;
use command_id::*;
use std::net::UdpSocket;
use std::time::Duration;
use std::thread;
use bincode::{serialize,deserialize};
use cubeos_service::{Command,Generic};
// use std::convert::TryFrom;

command_id!{
    Ping,
    Get,
    Set,
}

fn main() -> Result<(),CubeOSError>{
    let socket = UdpSocket::bind("0.0.0.0:9029")?;

    // let cmd_id: u16 = u16::try_from(CommandID::Ping)?;
    // let msg: Vec<u8> = cmd_id.to_be_bytes().to_vec();
    let mut msg: Vec<u8> = Command::<CommandID,Generic>::serialize(CommandID::Ping,Generic::new())?;

    println!("{:?}",msg);

    let mut buf = [0u8,5];
    socket.connect("0.0.0.0:8029");
    match socket.send(&msg) {
        Ok(_) => {
            match socket.recv(&mut buf) {
                Ok(m) => println!("{:?}", &buf[..m]),
                Err(_) => println!("Error"),
            }
        }
        Err(_) => println!("Error"),
    }

    msg.clear();
    let get = ExampleEnum::All;

    msg = Command::<CommandID,ExampleEnum>::serialize(CommandID::Get,get)?;

    println!("{:?}",msg);

    let mut buf = [0u8,5];
    socket.connect("0.0.0.0:8029");
    match socket.send(&msg) {
        Ok(_) => {
            match socket.recv(&mut buf) {
                Ok(m) => println!("{:?}", &buf[..m]),
                Err(_) => println!("Error"),
            }
        }
        Err(_) => println!("Error"),
    }

    msg.clear();
    let sub = ExampleInput{
        in_no: 10,
        in_no1: 256,
        in_no2: 0,
        in_str: "example".to_string(),
        in_bool: true,
    };
    let choice = ExampleEnum::Zero;
    let set = (sub,choice);

    msg = Command::<CommandID,(ExampleInput,ExampleEnum)>::serialize(CommandID::Set,set)?;

    println!("{:?}",msg);

    let mut buf = [0u8,5];
    socket.connect("0.0.0.0:8029");
    match socket.send(&msg) {
        Ok(_) => {
            match socket.recv(&mut buf) {
                Ok(m) => println!("{:?}", &buf[..m]),
                Err(_) => println!("Error"),
            }
        }
        Err(_) => println!("Error"),
    }

    Ok(())
}
