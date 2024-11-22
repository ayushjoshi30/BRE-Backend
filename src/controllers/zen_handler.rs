use warp::{Rejection, Reply};
use std::sync::Arc;
use serde_json::{json};
use zen_engine::{model::DecisionContent, DecisionEngine};
use sea_orm::DatabaseConnection;
pub async fn evaluate_decision_handler(db_pool: Arc<DatabaseConnection>) -> Result<impl Reply, Rejection> {
    // Create a simple JSON object
    let decision_content: Result<DecisionContent, _> = serde_json::from_str(include_str!("jdm_graph.json"));
    match decision_content {
        Ok(content) => {
            let engine = DecisionEngine::default(); // Create a default engine
            let decision = engine.create_decision(content.into()); // Create a decision using the engine

            // Evaluate decision with JSON input data, convert into Variable using `.into()`
            let result = decision.evaluate(json!( {
                "bureaus": {
                    "score": 600
                },
                "OAV": {
                    "value": 12
                },
                "customer": {
                    "Age": 23,
                    "Profile": "",
                    "DOB": "",
                    "Loanamountrequested": 800000
                },
                "Value": {
                    "Gridvalue": 20,
                    "Insurance value": 20,
                    "Market value": 20
                }
            }).into()).await;

            // Print result
            println!("Decision result: {:?}", result);
        }
        Err(e) => {
            eprintln!("Failed to deserialize JSON: {}", e);
        }
    }
    let response = serde_json::json!({
        "message": "hi"
    });

    // Return the JSON response
    Ok(warp::reply::json(&response))
}
