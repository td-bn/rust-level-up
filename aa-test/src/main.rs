use tracing::{info, instrument};
use tracing_subscriber;
use color_eyre::eyre;
use rand::{Rng, thread_rng};

#[tokio::main]
async fn main() -> eyre::Result<()>{
    color_eyre::install()?;

    tracing_subscriber::fmt::init();
    println!("Hello, world!");

    let s = bar().await?;
    Ok(println!("{}", s))
}

#[instrument]
pub async fn foo() -> eyre::Result<String> {

    let mut rng = thread_rng();
    let i: u32 = rng.gen();

    info!("Generated num:{}", i);
    match i%2 {
        0 => {
            info!("Returning info from foo()");
            Ok("String from async".to_owned())
        },
        _ => Err(eyre::eyre!("Unexpected error"))
        
    }
}

#[instrument]
pub async fn bar() -> eyre::Result<String> {
    foo().await
}
