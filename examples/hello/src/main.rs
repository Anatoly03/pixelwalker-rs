use pixelwalker::{
    Client,
    api::{User, World},
};
use std::{format, println};

fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();
    let client = Client::new().auth_with_email_password()?;

    // let worlds = client.list_all_iterative::<World>()?;
    // let users = client.list_all_iterative::<User>()?;

    // General Stats
    println!("# General Stats");
    println!();
    println!("- Total Worlds: {}", client.collection::<World>().len()?);
    println!("- Total Users: {}", client.collection::<User>().len()?);
    println!();

    println!("# Most Wooted Worlds");
    println!();
    println!("| {0:>7} | Woots | Title{:>25} |", "");
    println!("|:{0:->7}:|:{0:->5}:|:{:->30}-|", "");
    for (idx, world) in client
        .collection::<World>()
        .sort("-woots")
        .sort("-plays")
        .take(20)?
        .iter()
        .enumerate()
    {
        println!("| {:>7} | {:>5} | {:<30} |", format!("**{}.**", idx + 1), world.woots, world.title);
    }
    println!();

    println!("# Most Played Worlds");
    println!();
    println!("| {0:>7} | Plays | Title{:>25} |", "");
    println!("|:{0:->7}:|:{0:->5}:|:{:->30}-|", "");
    for (idx, world) in client
        .collection::<World>()
        .sort("-plays")
        .take(20)?
        .iter()
        .enumerate()
    {
        println!("| {:>7} | {:>5} | {:<30} |", format!("**{}.**", idx + 1), world.plays, world.title);
    }
    println!();
    Ok(())
}
