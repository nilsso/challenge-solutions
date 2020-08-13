use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Frequencies = HashMap<char, usize>;

/// Message encapsulation to send threads.
enum Message {
    Line(String),
    Join,
}

/// Get character frequency from a single line.
pub fn frequency_line(line: &str) -> Frequencies {
    let mut frequencies = Frequencies::new();
    for c in line
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
    {
        *frequencies.entry(c).or_insert(0) += 1;
    }
    frequencies
}

/// Get character frequency from an array of lines.
pub fn frequency(lines: &[&str], worker_count: usize) -> Frequencies {
    let mut total_frequencies = Frequencies::new();

    // Need two channels:
    // 1. to send workers messages (i.e. lines of text, and the eventual termination instruction);
    // 2. to receive frequency results from the workers.
    let (message_sender, message_receiver) = mpsc::channel();
    let (frequencies_sender, frequencies_receiver) = mpsc::channel();

    // Each thread needs a reference to the 1. message receiver, and 2. frequency sender.
    // - Arc allows each worker to have shared ownership of the channels
    // - Mutex allows each worker to have mutable access to the channels
    let message_receiver = Arc::new(Mutex::new(message_receiver));
    let frequencies_sender = Arc::new(Mutex::new(frequencies_sender));

    for _id in 0..worker_count {
        // Creating shared ownership references to the channels.
        let message_receiver = message_receiver.clone();
        let frequencies_sender = frequencies_sender.clone();

        // Since threads are going to be running for as long as there are lines to process,
        // individually receiving lines as they're ready, we need a handle so that we can later
        // join the threads (after having sent all threads the command to break).
        thread::spawn(move || loop {
            // Lock the receiver mutex to receive a message (this blocks the thread).
            let msg = message_receiver.lock().unwrap().recv().unwrap();

            // Message is either:
            // 1. a line, whose frequency result is sent back via the sender, or
            // 2. the termination instruction.
            match msg {
                Message::Line(line) => {
                    let res = frequency_line(line.as_str());

                    frequencies_sender
                        .lock()
                        .unwrap()
                        .send(res)
                        .unwrap();
                }
                Message::Join => break,
            }
        });
    }

    // Send out the lines.
    for line in lines {
        message_sender.send(Message::Line(line.to_string())).unwrap();
    }

    // Send out the termination instruction.
    for _ in 0..worker_count {
        message_sender.send(Message::Join).unwrap();
    }

    // Receive and assemble all the thread results.
    for frequencies in frequencies_receiver.iter().take(lines.len()) {
        for (c, n) in frequencies {
            *total_frequencies.entry(c).or_insert(0) += n;
        }
    }

    total_frequencies
}
