use colored::Colorize;
use std::process::Command;
use update::VERSION;

mod update;

// TODO add autoremove command
// TODO fix stats command

fn run(cmd: &str, args: &[&str]) {
    let status = Command::new(cmd).args(args).status().unwrap_or_else(|_| {
        eprintln!("{}", format!("Failed to execute {}", cmd).red());
        std::process::exit(1);
    });
    if status.success() {
        println!("{}", "Finished with no errors hooray!".green());
    }
}

fn require_pkg(pkg: Option<&String>) -> &str {
    pkg.map(|s| s.as_str()).unwrap_or_else(|| {
        eprintln!("{}", "Missing package name".red());
        std::process::exit(1);
    })
}

fn main() {
    colored::control::set_override(true);

    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("{}", "No command provided. Use: fusi <command>".red());
        std::process::exit(1);
    }

    let cmd = args[0].as_str();
    let pkg = args.get(1);

    match cmd {
        // fusi install <package>
        "install" => run("sudo", &["pacman", "-S", require_pkg(pkg)]),

        // fusi remove <package>
        "remove" => run("sudo", &["pacman", "-Rns", require_pkg(pkg)]),

        // does a light clean up unlike the remove command
        "softremove" => run("sudo", &["pacman", "-R", require_pkg(pkg)]),

        // searches for a package
        "search" => run("pacman", &["-Ss", require_pkg(pkg)]),

        // Updates the system
        "update" => run("sudo", &["pacman", "-Syu"]),

        // upgrades a pkg
        "upgrade" => run("sudo", &["pacman", "-S", require_pkg(pkg)]),

        // downgrades a pkg
        "downgrade" => run("sudo", &["pacman", "-U", require_pkg(pkg)]),

        // Gives you info abt a specific package
        "info" => run("pacman", &["-Si", require_pkg(pkg)]),

        // Checks if a specific pkg is installed
        "installed" => run("pacman", &["-Qs", require_pkg(pkg)]),

        // lists installed packages (not including deps)
        "list" => run("pacman", &["-Qe"]),

        // lists all installed packages including deps
        "listall" => run("pacman", &["-Q"]),

        // shows files owned by pkg
        "files" => run("pacman", &["-Ql", require_pkg(pkg)]),

        // shows which package owns a file
        "owner" => run("pacman", &["-Qo", require_pkg(pkg)]),

        // shows deps of pkg
        "deps" => run("pacman", &["-Si", require_pkg(pkg)]),

        // exact same as deps but fancier name
        "dependencies" => run("pacman", &["-Si", require_pkg(pkg)]),

        // shows install history
        "log" => run("cat", &["/var/log/pacman.log"]),

        // find, rank, update mirrorlist
        "mirrors" => run("sudo", &["reflector"]),

        // removes pacman lock file
        "unlock" => run("sudo", &["rm", "/var/lib/pacman/db.lck"]),

        // shows amount of pkgs installed
        "stats" => run("pacman", &["-Qq"]),

        // removes uused deps
        // "autoremove" => run("sudo", &["pacman", "-Qdtg"]),

        // Updates Fusi
        "self-update" => run("bash", &["-c", "curl -s https://raw.githubusercontent.com/fusiontech21/Fusi/main/Update/update.sh | bash"]),

        // FUN
        "secret" => {
            let txt = "You are secretly a Femboy";
            secrething(&txt);
        }

        // fusi details
        "details" => {
            println!("{}", r#"
                ███████╗██╗   ██╗███████╗██╗
                ██╔════╝██║   ██║██╔════╝██║
                █████╗  ██║   ██║███████╗██║
                ██╔══╝  ██║   ██║╚════██║██║
                ██║     ╚██████╔╝███████║██║
                ╚═╝      ╚═════╝ ╚══════╝╚═╝
            "#.cyan().bold());
            println!("{}", "A Tool to help beginners use the Terminal for Arch-based distros".white());
            println!("{}", format!("Version: {}", VERSION).white());
            println!("{}", "© 2025 fusiontech21 — AGPL-3.0".white());
        }

        // Help command                                     // THIS IS LOOKING GOOD REMASTERED THE FORMAT - Fusiontech
        "help" => {
            println!("{}", "Fusi - Available Commands".cyan().bold());
            println!("{}", "─────────────────────────────────────────".cyan());
            println!("{} {}", "fusi install <pkg>".green().bold(),      "→ Install a package");
            println!("{} {}", "fusi remove <pkg>".green().bold(),       "→ Remove a package (full cleanup)");
            println!("{} {}", "fusi softremove <pkg>".green().bold(),   "→ Remove just the package");
            println!("{} {}", "fusi search <pkg>".green().bold(),       "→ Search for a package");
            println!("{} {}", "fusi update".green().bold(),             "→ Update the entire system");
            println!("{} {}", "fusi upgrade <pkg>".green().bold(),      "→ Upgrade a specific package");
            println!("{} {}", "fusi downgrade <pkg>".green().bold(),    "→ Downgrade a package");
            println!("{} {}", "fusi info <pkg>".green().bold(),         "→ Show info about a package");
            println!("{} {}", "fusi installed <pkg>".green().bold(),    "→ Check if a package is installed");
            println!("{} {}", "fusi list".green().bold(),               "→ List explicitly installed packages");
            println!("{} {}", "fusi listall".green().bold(),            "→ List all installed packages");
            println!("{} {}", "fusi files <pkg>".green().bold(),        "→ Show files owned by a package");
            println!("{} {}", "fusi owner <file>".green().bold(),       "→ Show which package owns a file");
            println!("{} {}", "fusi deps <pkg>".green().bold(),         "→ Show dependencies of a package");
            println!("{} {}", "fusi stats".green().bold(),              "→ List all installed package names");
            println!("{} {}", "fusi log".green().bold(),                "→ Show pacman install history");
            println!("{} {}", "fusi mirrors".green().bold(),            "→ Update your mirrorlist");
            println!("{} {}", "fusi unlock".green().bold(),             "→ Remove pacman lock file");
            println!("{} {}", "fusi details".green().bold(),            "→ Show info about fusi");
            println!("{}", "─────────────────────────────────────────".cyan());
            println!("{}", "© 2025 fusiontech21 — AGPL-3.0".white());
        }

        // anything else
        _ => {
            println!("{}", format!("Unknown Command ({}) Type fusi help to list all Commands", cmd).yellow());
        }
    }

    update::checkupdate();
    std::process::exit(0);
}

fn secrething(txt: &str) {
    let colors = ["rd", "ylw", "grn", "cyn", "blue", "mgnt"];
    for (i, ch) in txt.chars().enumerate() {
        let clrs = match colors[i % colors.len()] {
            "rd"   => ch.to_string().red().bold(),
            "ylw"  => ch.to_string().yellow().bold(),
            "grn"  => ch.to_string().green().bold(),
            "cyn"  => ch.to_string().cyan().bold(),
            "blue" => ch.to_string().blue().bold(),
            "mgnt" => ch.to_string().magenta().bold(),
            _      => ch.to_string().white().bold(),
        };
        print!("{}", clrs);
    }
    println!();
}