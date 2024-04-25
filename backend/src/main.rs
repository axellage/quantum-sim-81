mod simulation;

use rocket::http::{ContentType, Method};
use rocket_cors::{AllowedOrigins, CorsOptions};

#[macro_use]
extern crate rocket;

use crate::simulation::circuit_validator::QuantumCircuitError;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use rocket::http::Status;
use rocket::response::{self, Responder, Response};
use rocket::Request;


use crate::simulation::circuit_parser::{UnparsedCircuit};
use crate::simulation::quantum_state::{QuantumState};

#[derive(Serialize, Deserialize)]
struct IncomingData {
    circuit_matrix: Vec<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Step {
    states: Vec<QuantumState>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ComplexContainer {
    re: f64,
    im: f64,
}

#[derive(Serialize, Deserialize)]
struct OutgoingData {
    state_list: Vec<QuantumState>,
}

#[derive(Debug, Serialize)]
struct ApiError {
    error: QuantumCircuitError,
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let error_json = serde_json::to_string(&self).unwrap();
        Response::build()
            .header(ContentType::JSON)
            .sized_body(None, std::io::Cursor::new(error_json))
            .status(Status::BadRequest)
            .ok()
    }
}

#[post("/simulate", format = "json", data = "<incoming_data>")]
fn simulate_circuit_handler(
    incoming_data: Json<IncomingData>,
) -> Result<Json<OutgoingData>, ApiError> {
    let matrix = incoming_data.into_inner().circuit_matrix;

    match simulation::simulator::simulate_circuit_handler(UnparsedCircuit { circuit: matrix }) {
        Ok(state_list) => {
            let mut step_list = Vec::new();

            for step in state_list {
                step_list.push(step);
            }


            let outgoing_data = OutgoingData { state_list: step_list };
            Ok(Json(outgoing_data))
        }
        Err(err) => Err(ApiError { error: err }),
    }
}

#[derive(Serialize, Deserialize)]
struct PingMessage {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct PingResponse {
    message: String,
}

#[post("/ping", format = "json", data = "<ping_message>")]
fn ping_handler(ping_message: Json<PingMessage>) -> Json<PingResponse> {
    let data: PingMessage = ping_message.into_inner();

    if data.message == "ping" {
        Json(PingResponse {
            message: "pong".parse().unwrap(),
        })
    } else {
        Json(PingResponse {
            message: "Huh?".parse().unwrap(),
        })
    }
}

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    rocket::build()
        .attach(cors.to_cors().unwrap())
        .mount("/", routes![simulate_circuit_handler, ping_handler])
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    /*#[test]
    fn test_simulate_single_qubit_gates() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let response = client
            .post("/simulate")
            .header(rocket::http::ContentType::JSON)
            .body(
                r#"{
                    "circuit_matrix": [
                        ["H", "I"],
                        ["I", "H"]
                    ]
                }"#,
            )
            .dispatch();

        //let expected_response = r#"{"state_list":[{"step":0,"state":[{"re":1.0,"im":0.0},{"re":0.0,"im":0.0},{"re":0.0,"im":0.0},{"re":0.0,"im":0.0}]},{"step":1,"state":[{"re":0.7071067811865475,"im":0.0},{"re":0.7071067811865475,"im":0.0},{"re":0.0,"im":0.0},{"re":0.0,"im":0.0}]},{"step":2,"state":[{"re":0.4999999999999999,"im":0.0},{"re":0.4999999999999999,"im":0.0},{"re":0.4999999999999999,"im":0.0},{"re":0.4999999999999999,"im":0.0}]}]}"#;
        let expected_response = r#"{"state_list":[{"states":[{"qubits":[0],"state":[{"re":1.0,"im":0.0},{"re":0.0,"im":0.0}]},{"qubits":[1],"state":[{"re":1.0,"im":0.0},{"re":0.0,"im":0.0}]}]},{"states":[{"qubits":[0],"state":[{"re":0.7071067811865475,"im":0.0},{"re":0.7071067811865475,"im":0.0}]},{"qubits":[1],"state":[{"re":1.0,"im":0.0},{"re":0.0,"im":0.0}]}]},{"states":[{"qubits":[0],"state":[{"re":0.7071067811865475,"im":0.0},{"re":0.7071067811865475,"im":0.0}]},{"qubits":[1],"state":[{"re":0.7071067811865475,"im":0.0},{"re":0.7071067811865475,"im":0.0}]}]}]}"#;
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some(expected_response.to_string()));
    }*/

    /*#[test]
    fn test_simulate_circuit_2() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let response = client
            .post("/simulate")
            .header(rocket::http::ContentType::JSON)
            .body(
                r#"{
                    "circuit_matrix": [
                        ["H", "CNOT-1"],
                        ["I", "CNOT-2"]
                    ]
                }"#,
            )
            .dispatch();

        let expected_response = r#"{"state_list":[{"step":0,"state":[{"re":1.0,"im":0.0},{"re":0.0,"im":0.0},{"re":0.0,"im":0.0},{"re":0.0,"im":0.0}]},{"step":1,"state":[{"re":0.7071067811865475,"im":0.0},{"re":0.7071067811865475,"im":0.0},{"re":0.0,"im":0.0},{"re":0.0,"im":0.0}]},{"step":2,"state":[{"re":0.7071067811865475,"im":0.0},{"re":0.0,"im":0.0},{"re":0.0,"im":0.0},{"re":0.7071067811865475,"im":0.0}]}]}"#;

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some(expected_response.to_string()));
    }*/
}
