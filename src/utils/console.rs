use crate::cli::matches::isDebug;
use crate::i18n::getLocaleText;
use crate::utils::util::writeLogFile;
use crate::LOG_PATH;
use chrono::Local;
use console::style;

pub enum ConsoleType {
    Info,
    Success,
    Warning,
    Err,
}

pub fn writeConsole(consoleType: ConsoleType, message: &str) {
    let title = match &consoleType {
        ConsoleType::Info => style(getLocaleText("Info", None)).cyan(),
        ConsoleType::Success => style(getLocaleText("Success", None)).green(),
        ConsoleType::Warning => style(getLocaleText("Warning", None)).yellow(),
        ConsoleType::Err => style(getLocaleText("Err", None)).red().on_black().bold(),
    };
    println!("  {}      {}", &title, message);
    if isDebug() {
        let time = Local::now().format("%T").to_string();
        // let tieme = SystemTime::now().duration_since(UNIX_EPOCH);
        writeLogFile(&LOG_PATH, &format!("{} {}  {}", time, console::strip_ansi_codes(&title.to_string()), message)).ok();
    }
}
