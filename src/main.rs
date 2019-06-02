use std::process::Command;

extern crate logwatcher;
use logwatcher::LogWatcher;

fn main() -> std::io::Result<()> {
    let mut log_watcher = LogWatcher::register("/var/log/pihole.log".to_string()).unwrap();

    log_watcher.watch(|line: String| {
        if line.contains("gravity") {
            println!("blocked: {}", &line[57..]);
            blink(1)
        } else {
            blink(0)
        }
    });

    Ok(())
}

fn blink(led_id: i8) {
    let arg = format!("echo 1 > /sys/class/leds/led{}/brightness; sleep 0.001; echo 0 > /sys/class/leds/led{}/brightness;", led_id, led_id);
    Command::new("bash")
        .arg("-c")
        .arg(arg)
        .output()
        .expect("failed change led value");
}
