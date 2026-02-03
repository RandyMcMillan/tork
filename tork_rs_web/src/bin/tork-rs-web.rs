use tork_rs_web::HealthCheck;

fn main() {
    let health_json_formatted = format!(r#"{{"status":"UP","version":"{}"}}"#, tork_rs_web::tork_rs::version());
    match HealthCheck::from_json(&health_json_formatted) {
        Some(hc) => {
            println!("Health Status: {}", hc.status);
            println!("Health Version: {}", hc.version);
            if hc.is_up() {
                println!("Service is UP!");
            } else {
                println!("Service is DOWN!");
            }
        }
        None => println!("Failed to parse health check JSON."),
    }
}
