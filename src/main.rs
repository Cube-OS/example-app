use example_api::*;
use cubeos_service::*;

app_macro!{
    example_service: Example{
        query: Get => fn get_values(&self, get: ExampleEnum) -> Result<ExampleOutput>; out: ExampleOutput;
        mutation: Set => fn set_values(&self, in_no: u16, in_n01: u32, in_no2: u16, in_str: String, in_bool: bool, choice: ExampleEnum) -> Result<()>;
    }
}

// This example App shows how to use the App macro to communicate with a service.
// If the Payload is connected to the computer the App is run on it is possible to use the API directly,
// but it is recommended to use the app_macro! as this allows for the communication with multiple Payloads.
// This also makes sure that any variable changes are saved to the Payload Service and logging is enabled.
// - performs a Telemetry request
// - overwrites variable of the service
// - performs a second Telemetry request to check the overwritten data
fn main() -> Result<()>{

    // Example get
    // This calls the get_values function of the service and returns the requested data.
    let get_values = Example::get_values(ExampleEnum::All).unwrap();

    // println!("{:?}", get_values);

    // Example set
    // This calls the set_values function of the service and overwrites the data.
    // The set function returns a Result<()> which is Ok(()) if the function was successful.
    // If the function was not successful it returns an ExampleError.
    let in_no = 1;
    let in_no1 = 2;
    let in_no2 = 3;
    let in_str = "test".to_string();
    let in_bool = true;
    let choice = ExampleEnum::One;
    Example::set_values(in_no, in_no1, in_no2, in_str, in_bool, choice)?;
    
    // Check that data has been overwritten
    let get_values_2 = Example::get_values(ExampleEnum::One).unwrap();

    if get_values_2.out_no == [1u16] && get_values_2.out_str == "test" && get_values_2.out_bool == [true] {
        println!("Data has been overwritten");
    } else {
        println!("Data has not been overwritten");
    }

    Ok(())
}
