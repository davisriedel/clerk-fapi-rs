use clerk_fapi_rs::{clerk::Clerk, configuration::ClerkFapiConfiguration};
use dotenv::dotenv;
use std::{env, io::{self, Write}};
use tokio::time::sleep;
use std::time::Duration;

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file if present
    dotenv().ok();

    // Get the PUBLIC_KEY from environment variables
    let public_key = env::var("PUBLIC_KEY").expect("PUBLIC_KEY environment variable is required");

    // Create configuration
    let config = ClerkFapiConfiguration::new(
        public_key,
        None, // Use default API URL
        None, // Use default store
    )?;

    // Initialize Clerk client
    let clerk = Clerk::new(config);

    // Load the client (this fetches initial data)
    let clerk = clerk.load().await?;

    println!("Welcome to the Clerk authentication example!");
    println!("Please select your sign-in method:");
    println!("1. Email Code");
    println!("2. Ticket");

    let choice = read_input("Enter your choice (1 or 2): ");

    match choice.as_str() {
        "1" => {
            // Email Code flow
            let email = read_input("Please enter your email address: ");
            
            // Create sign-in attempt
            let sign_in_response = clerk.api_client()
                .create_sign_in(
                    Some("email_code"),
                    Some(&email),
                    None, // password
                    None, // ticket
                    None, // redirect_url
                    None, // action_complete_redirect_url
                    None, // transfer
                    None, // code
                    None, // token
                )
                .await?;

            let sign_in_id = sign_in_response.response.id;

            println!("We've sent a verification code to your email.");
            println!("Please check your inbox and enter the code below.");
            
            let code = read_input("Enter verification code: ");

            // Attempt first factor verification
            let verification_response = clerk.api_client()
                .attempt_sign_in_factor_one(
                    &sign_in_id,
                    Some("email_code"),
                    Some(&code),
                    None, // password
                    None, // signature
                    None, // redirect_url
                    None, // action_complete_redirect_url
                    None, // ticket
                )
                .await?;

            if verification_response.response.status == clerk_fapi_rs::models::client_period_sign_in::Status::Complete {
                println!("Sign in successful!");
            } else {
                println!("Sign in failed. Status: {:?}", verification_response.response.status);
                return Ok(());
            }
        }
        "2" => {
            // Ticket flow
            let ticket = read_input("Please enter your ticket: ");
            
            let sign_in_response = clerk.api_client()
                .create_sign_in(
                    Some("ticket"),
                    None, // identifier
                    None, // password
                    Some(&ticket),
                    None, // redirect_url
                    None, // action_complete_redirect_url
                    None, // transfer
                    None, // code
                    None, // token
                )
                .await?;

            if sign_in_response.response.status == clerk_fapi_rs::models::client_period_sign_in::Status::Complete {
                println!("Sign in successful!");
            } else {
                println!("Sign in failed. Status: {:?}", sign_in_response.response.status);
                return Ok(());
            }
        }
        _ => {
            println!("Invalid choice!");
            return Ok(());
        }
    }

    // Give some time for the client to update
    sleep(Duration::from_millis(500)).await;

    // Get and display user information
    if let Some(user) = clerk.user().await {
        println!("\nUser Information:");
        println!("Name: {:?} {:?}", 
            user.first_name.unwrap_or_default(), 
            user.last_name.unwrap_or_default()
        );
        
        if let Some(email_addresses) = user.email_addresses {
            if !email_addresses.is_empty() {
                println!("Email: {}", email_addresses[0].email_address);
            }
        }
    } else {
        println!("Could not retrieve user information");
    }

    Ok(())
}
