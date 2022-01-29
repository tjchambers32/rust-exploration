use reqwest;
use std::error::Error;
use std::thread;

const NTHREADS: usize = 20;
const USERNAME: &str = "tjchambers";

// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = format!("https://challenge.hsiao.dev/03/u/{USERNAME}/passwords.txt");

    let body: String = reqwest::get(url)
    .await?
    .text()
    .await?;

    //TODOm read https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
    let pwds: Vec<&str> = body.split("\n").collect();
    // println!("{:#?}", pwds);

    //TODO figure out how to have a shared count for progress
    // let mut count = 0;

    // Make a vector to hold the children which are spawned.
    let mut children = vec![];

    for thread_num in 0..NTHREADS {
        //TODO 
        // split pwds into chunks for each thread
        let chunk_size = pwds.len() / NTHREADS;
        let start_index = thread_num * chunk_size;
        let end_index = (thread_num+1) * chunk_size;
        let pwds_chunk: Vec<&str> = pwds[start_index..end_index].to_vec();
        
        // Spin up another thread
        children.push(thread::spawn(move || {
            println!("this is thread number {}", thread_num);

            check_pwds(pwds_chunk);
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }

    Ok(())
}

async fn check_pwds(pwds: Vec<&str>) -> Result<&str, Box<dyn Error>> {
    let mut found_pwd = "";
    for pwd in pwds {
        let check_pwd_url = format!("https://challenge.hsiao.dev/03/u/{USERNAME}/check/{pwd}");
        let check_body = reqwest::get(check_pwd_url)
        .await?
        .text()
        .await?;

        // println!("{:#?}", check_body);

        match check_body.as_str() {
            "False" => (),
            "True" => {
                println!("Password found! <<>> {pwd}");
                found_pwd = pwd;
                break;
            }
            _ => println!("something else!"),
        }

        // count += 1;
        // if count % 10 == 0 {
        //     println!("{:#?}", count);
        // }
    }
    Ok(found_pwd)
}