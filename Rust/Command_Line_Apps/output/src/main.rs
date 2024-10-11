use indicatif::ProgressBar;

fn do_hard_work() {
    // Simulate some work
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("Doing hard work...");
}

fn main() {
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        do_hard_work();
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}
