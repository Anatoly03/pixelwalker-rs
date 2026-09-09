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
    println!("| {0:>7} | Woots | Title{0:>25} | Owner{0:>15} |", "");
    println!("|:{0:->7}:|:{0:->5}:|:{0:->30}-|:{0:->20}-|", "");
    for (idx, world) in client
        .collection::<World>()
        .sort("-woots")
        .sort("-plays")
        .take(20)?
        .iter()
        .enumerate()
    {
        let owner = client.collection::<User>().view(&world.owner)?;
        println!("| {:>7} | {:>5} | {:<30} | {:>20} |", format!("**{}.**", idx + 1), world.woots, world.title, owner.username);
    }
    println!();

    println!("# Most Played Worlds");
    println!();
    println!("| {0:>7} | Plays | Title{0:>25} | Owner{0:>15} |", "");
    println!("|:{0:->7}:|:{0:->5}:|:{0:->30}-|:{0:->20}-|", "");
    for (idx, world) in client
        .collection::<World>()
        .sort("-plays")
        .take(20)?
        .iter()
        .enumerate()
    {
        let owner = client.collection::<User>().view(&world.owner)?;
        println!("| {:>7} | {:>5} | {:<30} | {:>20} |", format!("**{}.**", idx + 1), world.plays, world.title, owner.username);
    }
    println!();
    Ok(())
}
