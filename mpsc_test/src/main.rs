use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn sleep_sender(name: &str, sender: mpsc::Sender<String>) {
    for i in 1..=5 {
        let msg = format!("{}: {}", name, i);
        sender.send(msg).unwrap();
        println!("送信しました: {}", name);
        thread::sleep(Duration::from_millis(1000))
    }
    sender.send("quit".to_string()).unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    let sender = tx.clone();
    thread::spawn(|| {
        sleep_sender("太郎", sender);
    });

    let sender = tx.clone();
    thread::spawn(|| {
        sleep_sender("次郎", sender);
    });

    loop {
        let buf = rx.recv().unwrap();
        println!("[受信] {}", buf);
        thread::sleep(Duration::from_secs(3));
        if buf == "quit" { break; }
    }
}
