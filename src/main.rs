use axum::{
    routing::{post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use transaction_core::Transaction;

#[derive(Deserialize)]
struct PaymentRequest {
    amount: f64,
    currency: String,
}

#[derive(Serialize)]
struct PaymentResponse {
    transaction_id: String,
    status: String,
}

async fn process_payment(
    Json(payment): Json<PaymentRequest>,
) -> Json<PaymentResponse> {
    let transaction = Transaction::new(payment.amount, payment.currency);

    let status = if transaction.validate() {
        "accepted"
    } else {
        "rejected"
    };

    Json(PaymentResponse {
        transaction_id: transaction.id.to_string(),
        status: status.to_string(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/payment", post(process_payment));
    println!("Payment processor starting");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await.unwrap();
    axum::serve(listener, app).await.unwrap();
}