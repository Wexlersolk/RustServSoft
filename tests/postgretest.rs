use uuid::Uuid;
use letsgetrusty::configuration::{get_configuration, DatabaseSettings};
use letsgetrusty::startup::run;
 
 
#[tokio::test]
async fn successful_connection_to_database() {
    let mut configuration = get_configuration().expect("Failed to read configuration.");
    println!("{:?}", configuration.database);
}  
 
  
