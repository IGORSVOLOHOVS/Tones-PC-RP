use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{serde_as, DeserializeAs, SerializeAs}; 
use std::fs;
use reqwest;
use serde_json::to_string;

#[derive(Serialize, Deserialize, Debug)]
struct LiquidType {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Liquid {
    #[serde(rename = "type")] // Handle renaming
    liquid_type: LiquidType,
    id: i32,
    name: String,
    usedCold: bool,
    toxic: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct TemperatureChangeParams {
    source: i32,
    target: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct WashingParams {
    liquid: Liquid,
    iters: i32,
    incubation: i32,
    temperature: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct LiquidApplicationParams {
    autoWash: bool,
    liquid: Liquid,
    incubation: i32,
    temperature: i32,
}

// Using an enum for flexibility with different step types
#[derive(Serialize, Deserialize, Debug)]
enum StepParams {
    Other(serde_json::Value), // Catch-all for unknown variants
}

#[derive(Serialize, Deserialize, Debug)]
struct Step {
    #[serde(rename = "type")]
    step_type: String,
    id: i32,
    params: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
struct DefaultWash {
    iters: i32,
    incubation: i32,
    liquid: Liquid,
    temperature: Option<i32>, // Temperature can be null
}

#[derive(Serialize, Deserialize, Debug)]
struct Protocol {
    id: i32,
    name: String,
    customLiquids: Vec<Liquid>,
    description: String,
    steps: Vec<Step>,
    creationDate: String,
    defaultWash: DefaultWash,
    author: Option<String>,
}

struct ProtocolManager{
    protocol_id: i32,
    step_id: i32,
    status: String,

    url: String,
    client: reqwest::blocking::Client,
    msg: serde_json::Value,
}

impl ProtocolManager {
    fn new(url: &str) -> Self {
        ProtocolManager {
            protocol_id: -1,
            step_id: -1,
            status: "OK".to_string(),
            url: url.to_string(),
            client: reqwest::blocking::Client::new(),
            msg: serde_json::json!({
                "protocol_id": -1,
                "step_id": -1,
                "status": "OK" // ERROR or OK
            }),
        }
    }

    fn run(&mut self) {
        let protocols = self.get_protocols().unwrap();
        for protocol in protocols.iter() {
            self.protocol_id = protocol.id;
            for step in protocol.steps.iter() {
                self.step_id = step.id;
                match step.step_type.as_str() {
                    "Temperature Change" => {                    
                        // Get the params
                        let params = serde_json::from_value::<TemperatureChangeParams>(step.params.clone()).unwrap();
                        // Do something with the params
                        // Send state
                        self.log(protocol.id, step.id, "OK");
                    },
                    "Washing" => {
                        // Get the params
                        let params = serde_json::from_value::<WashingParams>(step.params.clone()).unwrap();
                        // Do something with the params
                        // Send state
                        self.log(protocol.id, step.id, "OK");
                    },
                    "Liquid Application" => {
                        // Get the params
                        let params = serde_json::from_value::<LiquidApplicationParams>(step.params.clone()).unwrap();
                        // Do something with the params
                        // Send state
                        self.log(protocol.id, step.id, "OK");
                    },
                    _ => {
                        self.log(protocol.id, step.id, "ERROR: Unknown step type!");
                    }
                }
            }
        }

    }

    fn log(&mut self, protocol_id: i32, step_id: i32, status: &str) {
        self.msg["protocol_id"] = Value::from(protocol_id);
        self.msg["step_id"] = Value::from(step_id);
        self.msg["status"] = Value::from(status);

        let json_str = to_string(&self.msg).unwrap();

        let res = self.client.post(&self.url)
            .body(json_str)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .unwrap();
    }

    fn get_protocols(&mut self) -> Result<Vec<Protocol>, reqwest::Error> {
        let mut protocols: Vec<Protocol> = {  // serialize data into struct
            let res = fs::read_to_string("test.json").expect("Can't read file");
            serde_json::from_str::<Vec<Protocol>>(&res).unwrap()
        };
        Ok(protocols)
    }
}

fn main() {
    
    // Create a logger instance
    let mut manager = ProtocolManager::new("http://127.0.0.1:5000");
    
    // Run the manager
    manager.run();
}