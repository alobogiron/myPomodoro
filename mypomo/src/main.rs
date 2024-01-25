use std::io::{self};
use chrono::{Duration};
use tokio::time::sleep;

#[tokio::main]

async fn main(){
    println!("Welcome to the pomodoro timer!");
    let mut paused = false;

        let user_input_task = tokio::spawn(async move {
            loop{
                println!("Press 'Enter' to start a Pomodoro 'p' to pause and 'q' to quit.");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");

                match input.trim(){
                    "q" => {
                    println!("See you soon");
                    break;
                     }
                    "p" => {
                        paused = !paused;
                        println!("{}", if paused {"Paused"} else {"Resumed"});
                    }
                     _ => {
                         if !paused{
                        start_pomodoro().await;
                         }
                    }
                }
             }
        });
    //Wait for the user input task to complete
    user_input_task.await.expect("Error in user input task");

        
}

async fn start_pomodoro(){
    let work_duration = Duration::minutes(2);
    let break_duration = Duration::minutes(1);

    //Work session
    println!("Work for {work_duration} minutes");
    sleep(work_duration.to_std().unwrap()).await;

    //Break Session
     println!("Take a {break_duration} minutes of break");
    sleep(break_duration.to_std().unwrap()).await;
}


