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

#[derive(Serialize, Deserialize)]
struct IncomingData {
    circuit_matrix: Vec<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
struct Step {
    step: usize,
    state: Vec<ComplexContainer>,
}

#[derive(Serialize, Deserialize)]
struct ComplexContainer {
    re: f64,
    im: f64,
}

#[derive(Serialize, Deserialize)]
struct OutgoingData {
    state_list: Vec<Step>,
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
    let binding = incoming_data.into_inner();

    let matrix = binding
        .circuit_matrix
        .iter()
        .map(|row| row.iter().map(|item| item.as_str()).collect())
        .collect();

    match simulation::simulator::simulate_circuit(matrix) {
        Ok(state_list) => {
            let outgoing_data = OutgoingData { state_list };
            Ok(Json(outgoing_data))
        }
        Err(err) => Err(ApiError { error: err }),
    }
}

#[derive(Serialize, Deserialize)]
pub struct DefaultCircuitsResponse {
    circuits: Vec<Vec<Vec<String>>>,
}

impl DefaultCircuitsResponse {
    pub fn new() -> Self {
        DefaultCircuitsResponse {
            circuits: vec![Self::bell_state_circuit()]
        }
    }

    fn bell_state_circuit() -> Vec<Vec<String>> {
        let q0 = vec!["X".to_string(), "CNOT-1".to_string()];
        let q1 = vec!["I".to_string(), "CNOT-2".to_string()];

        vec![q0, q1]
    }
}

#[get("/default-circuits")]
fn get_default_circuits() -> Json<DefaultCircuitsResponse> {
    Json(DefaultCircuitsResponse::new())
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
        .mount("/", routes![simulate_circuit_handler, get_default_circuits])
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_simulate_circuit_1() {
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

        let expected_response = r#"{"state_list":[{"step":0,"state":[{"re":1.0,"im":0.0},{"re":0.0,"im":0.0},{"re":0.0,"im":0.0},{"re":0.0,"im":0.0}]},{"step":1,"state":[{"re":0.7071067811865475,"im":0.0},{"re":0.7071067811865475,"im":0.0},{"re":0.0,"im":0.0},{"re":0.0,"im":0.0}]},{"step":2,"state":[{"re":0.4999999999999999,"im":0.0},{"re":0.4999999999999999,"im":0.0},{"re":0.4999999999999999,"im":0.0},{"re":0.4999999999999999,"im":0.0}]}]}"#;

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some(expected_response.to_string()));
    }

    #[test]
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
    }
}
