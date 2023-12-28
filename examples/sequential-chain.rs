use dotenv::dotenv;
use llm_chain::parameters;
use llm_chain::step::Step;
use llm_chain::traits::Executor as ExecutorTrait;
use llm_chain::{chains::sequential::Chain, prompt};
use llm_chain_openai::chatgpt::Executor;
use std::io;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Create a new ChatGPT executor with the default settings
    let exec = Executor::new()?;

    // Create a chain of steps with two prompts
    let chain: Chain = Chain::new(vec![
        // First step: make a personalized birthday email
        Step::for_prompt_template(
            prompt!("You are a bot for making personalized greetings", "Make personalized birthday e-mail to the whole company for {{name}} who has their birthday on {{date}}. Include their name")
        ),

        // Second step: summarize the email into a tweet. Importantly, the text parameter becomes the result of the previous prompt.
        Step::for_prompt_template(
            prompt!( "You are an assistant for managing social media accounts for a company", "Summarize this email into a tweet to be sent by the company, use emoji if you can. \n--\n{{text}}")
        )
    ]);

    let name: String = get_name_input()?;

    // Run the chain with the provided parameters
    let res = chain
        .run(
            // Create a Parameters object with key-value pairs for the placeholders
            parameters!("name" => name, "date" => "February 30th 2023"),
            &exec,
        )
        .await
        .unwrap();

    // Print the result to the console
    println!("{}", res.to_immediate().await?.as_content());
    Ok(())
}

fn get_name_input() -> Result<String, Box<dyn std::error::Error>> {
    let mut input = String::new();
    println!("Enter a name:");
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
