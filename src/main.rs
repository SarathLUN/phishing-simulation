// src/main.rs
use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use dotenv::dotenv;
use mysql::*;
use mysql::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Mutex;

// Application state with database pool
struct AppState {
    pool: Mutex<Pool>,
}

// Structure for tracking click data
#[derive(Debug, Serialize, Deserialize)]
struct ClickRecord {
    email: String,
    ip_address: String,
    user_agent: String,
}

// Initialize database connection
fn init_db_pool() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    let opts = Opts::from_url(&database_url)
        .expect("Invalid database URL");

    Pool::new(opts).expect("Failed to create database pool")
}

// Create the necessary table if it doesn't exist
fn init_database(pool: &Pool) {
    let mut conn = pool.get_conn().expect("Failed to get DB connection");

    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS phishing_clicks (
            id INT AUTO_INCREMENT PRIMARY KEY,
            email VARCHAR(255) NOT NULL,
            ip_address VARCHAR(50) NOT NULL,
            user_agent TEXT NOT NULL,
            clicked_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    ).expect("Failed to create table");
}

// Landing page for the simulation - records the click and shows a generic page
async fn landing_page(
    state: web::Data<AppState>,
    query: web::Query<std::collections::HashMap<String, String>>,
    req: HttpRequest,
) -> impl Responder {
    // Extract email from query param
    let email = match query.get("email") {
        Some(email) => email,
        None => return HttpResponse::BadRequest().body("Invalid request"),
    };

    // Get IP address
    let ip = req.connection_info().peer_addr()
        .unwrap_or("unknown").to_string();

    // Get user agent
    let user_agent = req.headers().get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    // Record the click
    let click_record = ClickRecord {
        email: email.clone(),
        ip_address: ip,
        user_agent,
    };

    // Store in database
    let pool = &state.pool.lock().unwrap();
    let mut conn = pool.get_conn().expect("Failed to get database connection");

    conn.exec_drop(
        "INSERT INTO phishing_clicks (email, ip_address, user_agent) VALUES (?, ?, ?)",
        (click_record.email, click_record.ip_address, click_record.user_agent)
    ).expect("Failed to insert click record");

    // Return a generic response page
    HttpResponse::Ok().content_type("text/html").body(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Free Ride Promotion</title>
            <style>
                body { font-family: Arial, sans-serif; max-width: 600px; margin: 0 auto; padding: 20px; }
                .security-notice { margin-top: 30px; padding: 15px; background-color: #f5f5f5; border-radius: 5px; }
            </style>
        </head>
        <body>
            <h1>Security Awareness Training</h1>
            <p>This was a phishing simulation test conducted by your IT security team.</p>
            <div class="security-notice">
                <p><strong>Important:</strong> Your click has been recorded for training purposes.</p>
                <p>Remember to always verify the sender and be cautious about clicking links in emails, 
                   especially those offering promotions or requesting urgent action.</p>
            </div>
            <p>Thank you for participating in this security awareness exercise.</p>
        </body>
        </html>
    "#)
}

// Get statistics about who clicked (protected admin endpoint)
async fn admin_stats(state: web::Data<AppState>) -> impl Responder {
    let pool = &state.pool.lock().unwrap();
    let mut conn = pool.get_conn().expect("Failed to get database connection");

    // Query all clicks
    let clicks = conn
        .query_map(
            "SELECT email, ip_address, user_agent, clicked_at FROM phishing_clicks ORDER BY clicked_at DESC",
            |(email, ip_address, user_agent, clicked_at): (String, String, String, String)| {
                format!("Email: {}, IP: {}, User Agent: {}, Time: {}", email, ip_address, user_agent, clicked_at)
            },
        )
        .expect("Failed to query database");

    let response = clicks.join("\n\n");

    HttpResponse::Ok().content_type("text/plain").body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize database
    let pool = init_db_pool();
    init_database(&pool);

    // Start the server
    let port = env::var("PORT").unwrap_or_else(|_| "80".to_string());
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let bind_address = format!("{}:{}", host, port);

    println!("Starting phishing simulation server at http://{}", bind_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                pool: Mutex::new(pool.clone()),
            }))
            .route("/free-rides", web::get().to(landing_page))

    })
        .bind(bind_address)?
        .run()
        .await
}