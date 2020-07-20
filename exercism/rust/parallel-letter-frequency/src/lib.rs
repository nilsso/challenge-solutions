#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]

use crossbeam_utils::thread;
use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
//use std::thread;

type CountsT = HashMap<char, usize>;

enum Message<'a> {
    Line(&'a str),
    Join,
}

pub fn frequency<'a>(lines: &'static [&str], worker_count: usize) -> HashMap<char, usize> {
    let (line_sender, line_reciever) = mpsc::channel::<Message>();
    let (counts_sender, counts_reciever) = mpsc::channel::<CountsT>();

    let line_reciever = Arc::new(Mutex::new(line_reciever));
    let counts_sender = Arc::new(Mutex::new(counts_sender));

    for id in 0..worker_count {
        thread::scope(|s| {
            let line_reciever = line_reciever.clone();
            let counts_sender = counts_sender.clone();

            s.spawn(move |_| {
                let mut inner_counts = CountsT::new();

                loop {
                    let msg = line_reciever.lock().unwrap().recv().unwrap();
                    use Message::{Join, Line};

                    println!("thread {} waiting", id);

                    match msg {
                        Line(line) => {
                            println!("thread {} recieved '{}'", id, line);
                        }
                        Join => {
                            println!("thread {} joining", id);
                            break;
                        }
                    };
                }
            });
        })
        .ok();
    }

    for line in lines {
        line_sender.send(Message::Line(line)).ok();
    }

    for line in lines {
        line_sender.send(Message::Join).ok();
    }

    //while let Ok(inner_counts) = counts_reciever.recv() {}

    HashMap::new()
}
