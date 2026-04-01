/* UPDATE SYSTEM DONT EDIT THIS (unless you can make it better)
ALSO KEEP THIS IN THE GITHUB VERSION AND REMOVE IT IN THE AUR VERSION */
use colored::Colorize;

pub const VERSION: &str = "0.3.5";

pub fn checkupdate() {
    let url = "https://api.github.com/repos/fusiontech21/Fusi/releases/latest";

    let clint = reqwest::blocking::Client::builder()
        .user_agent("fusi")
        .build()
        .unwrap();

    if let Ok(resp) = clint.get(url).send() {
        if let Ok(txt) = resp.text() {
            if let Some(tg) = txt.split("\"tag_name\":\"").nth(1) {
                let latst = tg.split('"').next().unwrap_or("");
                if !latst.is_empty() && latst != VERSION {
                    println!(); 
                    println!(
                        "{}",
                        "╔══════════════════════════════════════════╗".yellow()
                    );
                    println!(
                        "{}",
                        format!("║  ⚠ New Version Available: {}  ║", latst)
                            .yellow()
                            .bold()
                    );
                    println!(
                        "{}",
                        "║  RUN 'fusi self-update' to update        ║".red().bold()
                    );
                    println!(
                        "{}",
                        "╚══════════════════════════════════════════╝".yellow()
                    );
                    println!();
                }
            }
        }
    }
}

pub fn latest() -> bool {
    let url = "https://api.github.com/repos/fusiontech21/Fusi/releases/latest";
    let client = reqwest::blocking::Client::builder()
        .user_agent("fusi")
        .build()
        .unwrap();

    if let Ok(resp) = client.get(url).send() {
        if let Ok(txt) = resp.text() {
            if let Some(tg) = txt.split("\"tag_name\":\"").nth(1) {
                let latst = tg.split('"').next().unwrap_or("");
                return latst == VERSION;
            }
        }
    }
    false
}