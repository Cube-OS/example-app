use example_api::*;
use command_id::*;
use rust_udp::*;
use cubeos_service::Command;
// use std::convert::TryFrom;

// CommandID Enum from Service can not be imported here, 
// copy-paste list of commands from service into this CommandID macro,
// to create a copy that can be used in the App
command_id!{
    //Keep these command IDs reserved, these commands are implemented in the CubeOS service
    Ping,
    Get,
    Set,
    //Add your commands here
}

// This example App 
// - pings the Example Service
// - performs a Telemetry request
// - overwrites variable of the service
// - performs a second Telemetry request to check the overwritten data
fn main() -> Result<(),CubeOSError>{

    // App IP + Service IP
    let app_host = "127.0.0.1:9029".to_string();
    let service = "127.0.0.1:8029".to_string();

    // The App opens a UDP Connection by binding the app_host IP
    // This connection can then be used to transfer commands
    let connection = Connection::from_path(app_host,service);

    // Create a command
    // 
    // Arguments:
    // `CommandID` -> command called
    // `Input` -> Input for Command, here ()
    let msg = Command::<CommandID,()>::serialize(CommandID::Ping,())?;
    
    println!("{:?}",msg);
    
    // Send command to Service and wait for reply
    //      
    // connection.transfer(command: Vec<u8>, rx_len: usize) -> Result<Vec<u8>>
    //
    // # Arguments:
    // command: Vec<u8> - Serialized Command to send to the service/payload
    // rx_len: usize - Length of read buffer/expected length of reply
    // 
    // # Output:
    // cubeos_error::Result<Vec<u8>>
    //
    // Output can be deserialized to any API struct or enum with
    // bincode::deserialize<E>(output)
    // where E is a struct or enum defined in the API
    //
    match connection.transfer(msg,1) {
        Ok(r) => println!("{:?}",r),
        Err(_) => println!("Error"),
    }

    // Create a command
    // 
    // Arguments:
    // `CommandID` -> command called
    // `Input` -> Input for Command, here ExampleEnum
    let get = ExampleEnum::All;
    let msg = Command::<CommandID,ExampleEnum>::serialize(CommandID::Get,get)?;

    println!("{:?}",msg);

    match connection.transfer(msg,20) {
        Ok(r) => println!("{:?}", r),
        Err(_) => println!("Error"),
    }

    // Create a command
    // 
    // Arguments:
    // `CommandID` -> command called
    // `Input` -> Input for Command, here ExampleInput
    let sub = ExampleInput{
        in_no: 10,
        in_no1: 256,
        in_no2: 0,
        in_str: "example".to_string(),
        in_bool: true,
    };
    let choice = ExampleEnum::Zero;
    let set = (sub,choice);
    let msg = Command::<CommandID,(ExampleInput,ExampleEnum)>::serialize(CommandID::Set,set)?;

    println!("{:?}",msg);

    match connection.transfer(msg,1) {
        Ok(r) => println!("{:?}", r),
        Err(_) => println!("Error"),
    }

    // Create a command
    // 
    // Arguments:
    // `CommandID` -> command called
    // `Input` -> Input for Command, here ExampleEnum
    let get = ExampleEnum::All;

    let msg = Command::<CommandID,ExampleEnum>::serialize(CommandID::Get,get)?;

    println!("{:?}",msg);

    match connection.transfer(msg,20) {
        Ok(r) => println!("{:?}", r),
        Err(_) => println!("Error"),
    }

    Ok(())
}
