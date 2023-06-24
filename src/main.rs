use std::process::Command;
use std::env;
use std::io;
use std::thread;
use std::time::Duration;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use rand::Rng;


fn main() {
    
    if cfg!(target_os = "windows") {
        let mut is_elevated = false;
        let mut elev: String = String::new();
            for arg in env::args() { // this always returns false for some reason, will fix later probs
                if arg == "--admin" {
                    is_elevated = true;
                    elev = "You HAVE admin".to_owned();
                }
                else {
                    is_elevated = false;
                    elev = "You do NOT have admin".to_owned();
                }

            }
        
        println!("{}", "        ,-.      . .       ,---.         .     
        |  \\     | |         |           |     
        |  | ,-. | |-  ,-:   |   ,-. ,-. | ,-. 
        |  / |-' | |   | |   |   | | | | | `-. 
        `-'  `-' ' `-' `-`-  '   `-' `-' ' `-' 
                                               ");
        println!("{}", "Made by DeltaZero, use for educational purposes only.");        
        println!("{} {}", "Welcome to DeltaTools,", elev);

        let mut expl: String = String::new();
        println!("Tools;");
        println!("{}", "1, CPU lagger (pretty slow)
2, Windows crasher (Requires admin)
3, WiFi information grabber (Gets WLAN info)");
        println!(".......................................................................................................................");
        io::stdin().read_line(&mut expl).expect("Failed");
        println!("{} {}", "executing tool", expl);

        if expl == "1\r\n"{
            while 1 == 1{
                let mut i = 0;
                while i != 999{
                    i+=1;
                    thread::spawn(||{
                        let mut rng = rand::thread_rng();
                        let numb: u64 = rng.gen();
                        let rooted: f64 = ((numb as f64).sqrt()).sqrt();     
                    });
                    
                }
            }
            thread::sleep(Duration::from_millis(1000)); // somewhere explain why this never stops, it should only run for 10 secs
            println!("done");
        }
        if expl == "2\r\n"{
            Command::new("cmd.exe")
                .args(&["/c", "taskkill /IM svchost.exe /F & taskkill /IM lsass.exe /F"]) // this is such bs
                .status()
                .expect("Failed");
        }
        if expl == "3\r\n"{
            let mut wifilist = Command::new("cmd.exe")
                .args(&["/c", "NETSH WLAN SHOW PROFILE"])
                .status()
                .expect("failed");
            println!("{}", wifilist);
            println!("Under 'User profiles', select one profile to get info from.");
            let mut profile: String = String::new();
            io::stdin().read_line(&mut profile).expect("Failed");

            let mut wifiinfo = Command::new("cmd.exe")
                .args(&["/c", "NETSH WLAN SHOW PROFILE",  &profile.trim(), "key=clear"])
                .status()
                .expect("failed");
            println!("{}", wifiinfo);
        }
        

    }
    

    
    
    
    

}
