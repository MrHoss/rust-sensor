use std::{time::Duration,process::Command};


pub fn clear_terminal() {
    if cfg!(windows) {
      let _ = Command::new("cmd").arg("/c").arg("cls").status();
    } else {
      let _ = Command::new("sh").arg("-c").arg("clear").status();
    }
}

pub async fn emit_beep(count: u64) {
    println!("Beep {}x", count);
    for i in 0..count {
      std::thread::sleep(Duration::from_millis((1000 / count) * i));
      println!("\x07");
    }
}

pub fn format_memory(value: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if value >= TB {
      format!("{:.2}GB", value as f64 / GB as f64)
    } else if value >= GB {
      format!("{:.2}GB", value as f64 / GB as f64)
    } else if value >= MB {
      format!("{:.2}MB", value as f64 / MB as f64)
    } else if value >= KB {
      format!("{:.2}KB", value as f64 / KB as f64)
    } else {
      format!("{} bytes", value)
    }
}
