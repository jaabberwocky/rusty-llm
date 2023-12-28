use dotenv::dotenv;
use llm_chain::{executor, parameters, prompt, step::Step};
use std::io;

// Declare an async main function
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the .env file
    dotenv().ok();

    // Create a new ChatGPT executor
    let exec = executor!()?;

    // Create our step containing our prompt template
    let step = Step::for_prompt_template(prompt!(
        "You are an expert greeting writer employed by Hallmark for making personalized greetings",
        "Make a personalized greeting tweet for {{text}}" // Text is the default parameter name, but you can use whatever you want
    ));

    let name = get_name_input()?;
    let res = step.run(&parameters!(name), &exec).await?;
    println!("{}", res);

    Ok(())
}

fn get_name_input() -> Result<String, Box<dyn std::error::Error>> {
    let mut input = String::new();
    println!("Enter a name:");
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
