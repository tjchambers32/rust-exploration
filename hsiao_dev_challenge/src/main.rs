use reqwest;
use std::error::Error;

// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let username = "tjchambers";
    let url = format!("https://challenge.hsiao.dev/03/u/{username}/passwords.txt");

    let body = reqwest::get(url)
    .await?
    .text()
    .await?;

    let body = body.split("\n");
    let pwds: Vec<&str> = body.collect();
    // println!("{:#?}", pwds);

    let mut count = 0;
    for pwd in pwds {
        let check_pwd_url = format!("https://challenge.hsiao.dev/03/u/{username}/check/{pwd}");
        let check_body = reqwest::get(check_pwd_url)
        .await?
        .text()
        .await?;

        // println!("{:#?}", check_body);

        match check_body.as_str() {
            "False" => (),
            "True" => {
                println!("Password found! <<>> {pwd}");
                break;
            }
            _ => println!("something else!"),
        }

        count += 1;
        if count % 10 == 0 {
            println!("{:#?}", count);
        }
        
    }


    Ok(())
}