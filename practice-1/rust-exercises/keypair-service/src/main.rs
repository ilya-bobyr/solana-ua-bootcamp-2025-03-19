use solana_sdk::signature::{Keypair, Signer};
use dotenv::dotenv;
use dotenv::from_filename;
use std::env;
use std::io;
use std::fs;
use std::time::Instant;

fn main() {
    dotenv().ok();
    loop {
        let mut action: String = String::new();

        println!("\x1b[32m----------------------------\x1b[0m");
        println!("Enter a command:");
        println!("\x1b[32m1\x1b[0m Generate and save keys");
        println!("\x1b[32m2\x1b[0m Read keys");
        println!("\x1b[32m0\x1b[0m Exit");

        match io::stdin().read_line(&mut action) {
            Ok(_) => {}
            Err(_e) => {
                println!("\x1b[31mInput error-{}\x1b[0m", _e)
            }
        }

        let command: u8 = action.trim().parse().unwrap();

        match &command {
            1 => { generate_keys(); },
            2 => { read_keys(); },
            0 => { break; },
            _ => println!("\x1b[31mInvalid command\x1b[0m\nUse only \x1b[32m0\x1b[0m, \x1b[32m1\x1b[0m or \x1b[32m2\x1b[0m"),
        }
    }
    
}


fn generate_keys() {
    let mut prefix: String = String::new();
    let mut keypair: Keypair = Keypair::new();

    println!("\x1b[32m----------------------------\x1b[0m");
    println!("Enter the desired prefix (3 characters max) or press enter to generate keys without a prefix:");
    match io::stdin().read_line(&mut prefix) {
        Ok(_pre) => {
            if _pre > 4 {
                println!("❌ Error: \x1b[91mPrefix is too long (allowed 3 characters max)\x1b[0m");
                return;
            } else if _pre > 1 {
                let mut attempts = 0;
                let start_time: Instant = Instant::now();
                let _prefix_value: &str = &prefix.trim();

                loop {
                    let _keypair_trial = Keypair::new();
                    let _pubkey_str = bs58::encode(_keypair_trial.pubkey().to_bytes()).into_string();
                    attempts += 1;
            
                    if _pubkey_str.starts_with(_prefix_value) {
                        println!("\x1b[32m----------------------------\x1b[0m");
                        println!("✅ Found a key with prefix \x1b[4;34m{}\x1b[0m", prefix);
                        println!("Attempts: {}", attempts);
                        println!("Time: {:.2?}", start_time.elapsed());

                        keypair = _keypair_trial;
                        break;
                    }
                    if attempts % 100_000 == 0 {
                        println!("Attempts: {}. Let's continue", attempts);
                    }
                }
            } 

        }

        Err(_e) => {
            println!("\x1b[31mInput error-{}\x1b[0m", _e)
        }
    }

    let private_key: String = bs58::encode(keypair.to_bytes()).into_string();
    let public_key: String = keypair.pubkey().to_string();

    println!("Public Key: \x1b[34m{}\x1b[0m", public_key);
    println!("Private Key: \x1b[34m{}\x1b[0m", private_key);

    let env_content: String = format!("PRIVATE_KEY={}\nPUBLIC_KEY={}\n", private_key, public_key);
    fs::write("./.env", env_content).expect("Failed to write to .env file");

    println!("✅ The keys saved to .env file");
    reload_env()
}

fn read_keys() {
    let private_key = match env::var("PRIVATE_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("❌ Error: \x1b[91mPRIVATE_KEY value not found in the env\x1b[0m");
            return;
        }
    };

    let _private_key_bytes: Vec<u8> = match bs58::decode(&private_key).into_vec() {
        Ok(bytes) if bytes.len() == 64 => bytes,
        _ => {
            eprintln!("❌ Error: \x1b[91mInvalid private key format\x1b[0m");
            return;
        }
    };
    
    let keypair: Keypair = match Keypair::from_bytes(&_private_key_bytes) {
        Ok(kp) => kp,
        Err(_) => {
            eprintln!("❌ Error: \x1b[91mFailed to create Keypair\x1b[0m");
            return;
        }
    };

    println!("✅ \x1b[95mPublic key:\x1b[0m \x1b[4;34m{}\x1b[0m", keypair.pubkey());
}


fn reload_env() {
    env::vars().for_each(|(key, _)| unsafe { env::remove_var(key) });
    from_filename(".env").ok();
}