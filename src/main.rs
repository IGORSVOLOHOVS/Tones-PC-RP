use serialport::{SerialPort, DataBits, StopBits, FlowControl};
use std::time::Duration;
use std::io::{BufRead, BufReader, Write};
use generator::Gn;
use std::thread::sleep;

// Function to report step status using a generator (unchanged)
fn report_step_status(step_name: &str) -> Gn<String> {
    Gn::new_scoped(|mut co| {
        co.yield_(format!("{} - Processing", step_name));
        sleep(Duration::from_secs(2)); // Simulate processing
        match step_name {
            "Step 3" => co.yield_(format!("{} - Error", step_name)), // Simulated error
            _ => co.yield_(format!("{} - OK", step_name)),
        }
    })
}

fn main() {
    let port_name = "/dev/ttyUSB0"; // Or your actual serial port
    let baud_rate = 9600;
    let data_bits = DataBits::Eight;
    let stop_bits = StopBits::One;
    let flow_control = FlowControl::None;

    // Open the serial port
    let mut port = serialport::open_with_settings(
        &port_name, 
        &serialport::SerialPortSettings {
            baud_rate,
            data_bits,
            stop_bits,
            flow_control,
            timeout: Duration::from_millis(1000), // Adjust timeout as needed
        }
    ).expect("Failed to open serial port");

    let mut reader = BufReader::new(&mut port);
    let mut buffer = String::new();

    loop {
        // ... (Reading commands from the serial port) ...

        match command_str {
            command if command.starts_with("W_") => {
                let mut step_generator = report_step_status("Waiting Command");
                waiting_command(command[2..].parse::<u64>().unwrap(), &mut port, &mut step_generator);
            },
            command if command.starts_with("BTC_") => {
                let mut step_generator = report_step_status("Blocking Temp Change");
                blocking_temperature_change(command[4..].parse::<f32>().unwrap(), &mut port, &mut step_generator);
            },
            command if command.starts_with("TC_") => {
                let mut step_generator = report_step_status("Non-Blocking Temp Change");
                nonblocking_temperature_change(command[3..].parse::<f32>().unwrap(), &mut port, &mut step_generator);
            },
            command if command.starts_with("LA_") => {
                let parts: Vec<&str> = command[3..].split('_').collect();
                let from = parts[0].parse::<u32>().unwrap();
                let to = parts[1].parse::<u32>().unwrap();
                let volume_ml = parts[2].parse::<u32>().unwrap();

                let mut step_generator = report_step_status("Liquid Application");
                liquid_application(from, to, volume_ml, &mut port, &mut step_generator);
            },
            _ => println!("Unknown command: {}", command_str)
        }
        buffer.clear();
    }
}

// ... (Command handling functions with generator integration) ...

fn waiting_command(time_ms: u64, port: &mut Box<dyn SerialPort>, step_generator: &mut Gn<String>) {
    // ... (Your waiting command logic) ...
    loop {
        if let Some(status) = step_generator.resume() {
            if let Err(e) = port.write_all(status.as_bytes()) {
                eprintln!("Error writing to serial port: {:?}", e);
            }
            if status.contains("Error") || status.contains("OK") {
                break;
            }
        } else {
            break; 
        }
    }
}

// ... (Similar implementations for blocking_temperature_change, nonblocking_temperature_change, and liquid_application) ...
