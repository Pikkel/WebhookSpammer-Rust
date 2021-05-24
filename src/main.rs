use std::io;
use std::collections::HashMap;

fn main() {

    let mut webhook = String::new();
    let mut message = String::new();
    let mut times = String::new();

    println!("Welcome to this webhook spammer!");
    println!("Enter a webhook to start spamming");
    io::stdin().read_line(&mut webhook).expect("Error invalid input");
    println!("Okey the webhook to spam is: {}", webhook);
    println!("Enter the message that you want to spam");
    io::stdin().read_line(&mut message).expect("Error invalid input");
    println!("key the message is: {}", message);
    println!("Now enter the number of webhooks that you want to spam!");
    io::stdin().read_line(&mut times).expect("Error invalid input");

    let timesint: i32 =  times.trim().parse::<i32>().expect("Invalid number");

    let client = reqwest::Client::new();

    let mut map = HashMap::new();
    map.insert("content", &message); 

    for i in 0..timesint {
        println!("Sending webhook number: {}", i);
        client.post(&webhook)
            .json(&map)
            .send();
    }
}
