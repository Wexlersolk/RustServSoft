use letsgetrusty::configuration::get_configuration;
 
 
#[tokio::test]
async fn successful_connection_to_database() {
    let configuration = get_configuration().expect("Failed to read configuration");
    println!("{:?}", configuration);
     
}  
 
  
