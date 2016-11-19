extern crate gst;

use std::io;
use std::io::prelude::*;
use std::thread;

fn player_loop(mut playbin : gst::PlayBin) {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        let mut line_split = line.split(' ');
        let command = line_split.next();
        let argument : Vec<&str> = line_split.collect();
        let argument = argument.join(" ");

        match command.unwrap() {
            "PAUSE" => {
                if playbin.is_playing() {
                    playbin.pause();
                    println!("INFO PLAYBACK paused");
                } else {
                    playbin.play();
                    println!("INFO PLAYBACK playing");
                }
            }
            "RESUME" if playbin.is_paused() => {
                playbin.play();
                println!("INFO PLAYBACK playing");
            }
            "LOAD" => {
                let track_path = String::from("file://")  + argument.as_str();

                println!("INFO LOADING {}", track_path);
                playbin.set_null_state();
                playbin.set_uri(&track_path);
                playbin.play();
                println!("INFO PLAYBACK playing");
            }
            "SEEK" => {
                let duration_s = playbin.duration_s().unwrap();
                let position_s = playbin.position_s().unwrap();
                let seek_param = String::from(argument);
                let (seek_operator, seek_rest) = seek_param.split_at(1);
                let seek_parsed_s = seek_rest.parse::<f64>().unwrap();
                let seek_position = 
                    match seek_operator {
                        "+" => position_s + seek_parsed_s,
                        "-" => position_s - seek_parsed_s,
                        "%" => duration_s * (seek_parsed_s * 0.01),
                        _ => seek_parsed_s // check bounds
                    };

                playbin.set_position_s(seek_position);
            }
            "VOLUME" => {
                let volume = argument.parse::<f64>().unwrap();
                playbin.set_volume(volume)
            }
            _ => {
                println!("ERROR unknown command '{}'", line);
            }
        }
    }

    // Exit on EOF
    std::process::exit(1);
}

fn main(){
    gst::init();
    let playbin = gst::PlayBin::new("audio_player").expect("Couldn't create playbin");
    let mut mainloop = gst::MainLoop::new();
    let mut bus = playbin.bus().expect("Couldn't get pipeline bus");
    let bus_receiver = bus.receiver();
    
    println!("INFO Started remote-player 0.1.0");
    mainloop.spawn();

    thread::spawn(move || player_loop(playbin));

    for message in bus_receiver.iter() {
        match message.parse(){
            gst::Message::ErrorParsed{ref error, ..} => {
                println!("ERROR GSTREAMER {}", error.message());
            }
            gst::Message::Eos(_) => {
                println!("INFO PLAYBACK stopped");
            }
            _ => {
                // println!("msg of type `{}` from element `{}`", message.type_name(), message.src_name());
            }
        }
    }
    mainloop.quit();
}
