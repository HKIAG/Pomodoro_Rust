
use std::env;
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::BufReader;
use rodio::Source;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    println!("Welcome to Danny's Pomodoro Application!");
    println!("{} Pomodoros - starting now", config.query);
    let reps = config.query.parse::<i32>().unwrap();
    for _i in 0..reps{
        basic_pom();
        rest();
    
    }

    println!("All pomodoros complete - goodbye!");
}

fn basic_pom(){
    thread::spawn(||{
        thread::sleep(Duration::from_millis(4700));
        println!("Pomodoro (25 mins) starting now - get working!");
    });
    play_sound("start.mp3", 7);
    thread::sleep(Duration::from_secs(5));
    countdown();
    play_sound("countdown.mp3", 10); 
    println!("Reached the end of the pomodoro!");
    play_sound("bang.mp3", 5);
}

fn countdown(){
    thread::spawn(|| {
        thread::sleep(Duration::from_secs(3));
        for i in (1..6).rev(){
            let val = ".".repeat(i);
            println!("{}{}", i, val);
            
            thread::sleep(Duration::from_millis(1500));
    }
    });
}
fn rest(){
    thread::spawn(||{
        play_sound("ahh.mp3", 6);
    });
    thread::sleep(Duration::from_secs(3));
    println!("Rest time (5 mins) starting now - relax!");
    thread::sleep(Duration::from_secs(5));
    countdown();
    play_sound("countdown.mp3", 10);
    println!("Rest time is over!");
}

fn play_sound(choice:&str, duration:u64){
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let file = File::open(choice).unwrap();
    let raw = rodio::Decoder::new(BufReader::new(file)).unwrap(); 
    stream_handle.play_raw(raw.convert_samples()).ok();
    thread::sleep(Duration::from_secs(duration));
}


struct Config{
    query:String, 
}

impl Config{
    fn new(args: &[String]) -> Config {
        if args.len() == 1{
            let query = "1".to_string();
            return Config{query}
        }
        else if args.len() > 2{
            panic!("Too many arguments");
        }
        else{
        let query = args[1].clone();
        Config{query} 
        }
    }
} 


