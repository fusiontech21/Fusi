use colored::Colorize;
use std::process::Command;
use update::VERSION;

mod update;

// TODO add autoremove command
// TODO fix stats command

fn run ( sudo: bool, args: Vec<&str>) {
    let mut cmd = if sudo {
        let mut c = Command::new("sudo");
        c.args(&args);
        c
    }else {
        let mut c = Command::new(&args[0]);
        c.args(&args[1..]);
        c
    };

    let sts = cmd.status().unwrap_or_else(|_| {
        eprintln!("{}", "Failed to execute Command".red());
        std::process::exit(1);
    });

    if sts.success(){
        println!("{}", "Finished with no errors!!".green())
    }
}

fn require_pkg(pkg: Option<&String>) -> &str {
    pkg.map(|s| s.as_str()).unwrap_or_else(|| {
        eprintln!("{}", "Missing package name".red());
        std::process::exit(1);
    })
}

fn hasaurhelper() -> Option<&'static str> {
    if let Ok(status) = Command::new("which").arg("paru").status() {
        if status.success() {
            return Some("paru");
        }
    }
    if let Ok(status) = Command::new("which").arg("yay").status() {
        if status.success() {
            return Some("yay");
        }
    }
    None
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
        // fusi install <package(s)>
        "install" => {
                let mut a = vec!["pacman", "-S"];
                for p in &args[1..] { a.push(p.as_str()); }
                run(true, a)
        }


        // removes a package and its deps command = fusi remove pkg
        "remove" => {
        let mut a = vec!["pacman", "-Rns"];
        for p in &args[1..] { a.push(p.as_str()); }
        run(true, a);
        }

        "test" => {println!("it works!");}

        // does a light clean up unlike the remove command
        "softremove" => {
        let mut a = vec!["pacman", "-R"];
        for p in &args[1..] { a.push(p.as_str()); }
        run(true, a);
        }

        // last 20 lines of installs
        "history" => run(false, vec!["tail", "-n", "20", "/var/log/pacman.log"]),

        // searches for a package
        "search" => run(false, vec!["pacman", "-Ss", require_pkg(pkg)]),

        // Updates the system
        "update" => run(true, vec!["pacman", "-Syu"]),

        // Force Upgrades the system (not recommended only for mirror issues and some other shit)
        "forceupdate" => run(true, vec!["pacman", "-Syyu"]),

        // upgrades a pkg
        "upgrade" => {
        let mut a = vec!["pacman", "-S"];
        for p in &args[1..] { a.push(p.as_str()); }
        run(true, a);
        }

        // downgrades a pkg
        "downgrade" => run(true, vec!["pacman", "-U", require_pkg(pkg)]),

        // Gives you info abt a specific package
        "info" => run(false, vec!["pacman", "-Si", require_pkg(pkg)]),

        // Check for borken deps
        "check" => run(false, vec!["pacman", "-Dk"]),

        // verify pkg isnt corrupt
        "verify" => run(false, vec!["pacman", "-Qk", require_pkg(pkg)]),

        // shows pacmans cache size
        "cache" => run(false, vec!["du", "-sh", "/var/cache/pacman/pkg"]),

        // clean old package cache
        "cleancache" => run(true,  vec!["pacman", "-Sc"]),

        // show packages nothing depends on
        "leaves" => run(false, vec!["pacman", "-Qdtt"]),

        // same as list but manually installed
        "explicit" => run(false, vec!["pacman", "-Qe"]),

        // show packages from aur
        "foreign" => run(false, vec!["pacman", "-Qm"]),
        
        // reinstall a package
        "reinstall"  => {
            let mut a = vec!["pacman", "-S"];
            for p in &args[1..] { a.push(p.as_str()); }
            run(true, a);
        }

        /* AUR  STUFF*/
        // Install pk from aur
            "aur" => {
                let hlpr = hasaurhelper().unwrap_or_else(|| {
                    eprintln!("{}", "No AUR helper found! Install Paru or Yay first.".red());
                    std::process::exit(1);
                });
                let mut a = vec![hlpr, "-S"];
                for p in &args[1..] { a.push(p.as_str()); }
                run(false, a); 
            }
        
        // updates a pkg from aur
        "aur-update" => {
            let hlpr = hasaurhelper().unwrap_or_else(|| {
                eprintln!("{}", "No AUR helper found! Install paru or yay first.".red());
                std::process::exit(1);
            });
            run(false, vec![hlpr, "-Sua"]);
}

        /* AUR  STUFF */

        // Checks if a specific pkg is installed
        "installed" => run(false, vec!["pacman", "-Qs", require_pkg(pkg)]),

        // lists installed packages (not including deps)
        "list" => run(false, vec!["pacman", "-Qe"]),

        // lists all installed packages including deps
        "listall" => run(false, vec!["pacman", "-Q"]),

        // shows files owned by pkg
        "files" => run(false, vec!["pacman", "-Ql", require_pkg(pkg)]),

     // shows which package owns a file
        "owner" => run(false, vec!["pacman", "-Qo", require_pkg(pkg)]),

        // shows deps of pkg
        "deps" => run(false, vec!["pacman", "-Si", require_pkg(pkg)]),

        // shows how much space a pkg usese
        "size" => run(false, vec!["pacman", "-Qi", require_pkg(pkg)]),

        // saves installed packages to file
        "backup" => run(false, vec!["bash", "-c", "pacman -Qe > ~/fusi-backup.txt && echo 'Backup saved to ~/fusi-backup.txt'"]),
        
        // reinstalls all packages from the backup
        "restore" => run(true, vec!["bash", "-c", "pacman -S --needed $(cat ~/fusi-backup.txt | awk '{print $1}')"]),

        // exact same as deps but fancier name
        "dependencies" => run(false, vec!["pacman", "-Si", require_pkg(pkg)]),

        // shows install history
        "log" => run(false, vec!["cat", "/var/log/pacman.log"]),

        // find, rank, update mirrorlist
        "mirrors" => run(true, vec!["reflector"]),

        // removes pacman lock file
        "unlock" => run(true, vec!["rm", "/var/lib/pacman/db.lck"]),

        // shows amount of pkgs installed
        "stats" => run(false, vec!["pacman", "-Qq"]),

        // Shows latest arch news (this feature got recommended by AI)
        "news" => run(false, vec!["bash", "-c", "curl -s https://archlinux.org/feeds/news/ | grep -oP '(?<=<title>)[^<]+' | head -10"]),

        // removes uused deps
        "autoremove" => {
        let output = Command::new("pacman")
            .args(["-Qdtq"])
            .output()
            .expect("Failed to get orphaned packages");

        if output.stdout.is_empty() {
            println!("{}", "No orphaned packages found!".green());
        } else {
            let pkgs = String::from_utf8(output.stdout).unwrap();
            let pkgls: Vec<&str> = pkgs.lines().collect();
            let mut cmdar = vec!["pacman", "-Rns"];
            cmdar.extend(pkgls.iter().map(|s| *s));
            run(true, cmdar);
        }
    }

        // Updates Fusi
        "self-update" => {
            run(false, vec!["bash", "-c", "curl -s https://raw.githubusercontent.com/fusiontech21/Fusi/main/Update/update.sh | bash"]);
            std::process::exit(0); // exit before checkupdate() runs
        }   
        
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
            println!("{}", "© 2026 fusiontech21 — AGPL-3.0".white());
        }

        // Help command                                     // THIS IS LOOKING GOOD REMASTERED THE FORMAT - Fusiontech
    "help" => {
        println!("{}", "Fusi - Available Commands".cyan().bold());
        println!("{}", "─────────────────────────────────────────".cyan());
        println!("{} {}", "fusi install <pkg>".green().bold(),        "→ Install a package");
        println!("{} {}", "fusi remove <pkg>".green().bold(),         "→ Remove a package (full cleanup)");
        println!("{} {}", "fusi softremove <pkg>".green().bold(),     "→ Remove just the package");
        println!("{} {}", "fusi reinstall <pkg>".green().bold(),      "→ Reinstall a package");
        println!("{} {}", "fusi search <pkg>".green().bold(),         "→ Search for a package");
        println!("{} {}", "fusi update".green().bold(),               "→ Update the entire system");
        println!("{} {}", "fusi upgrade <pkg>".green().bold(),        "→ Upgrade a specific package");
        println!("{} {}", "fusi downgrade <pkg>".green().bold(),      "→ Downgrade a package");
        println!("{} {}", "fusi info <pkg>".green().bold(),           "→ Show info about a package");
        println!("{} {}", "fusi installed <pkg>".green().bold(),      "→ Check if a package is installed");
        println!("{} {}", "fusi list".green().bold(),                 "→ List explicitly installed packages");
        println!("{} {}", "fusi listall".green().bold(),              "→ List all installed packages");
        println!("{} {}", "fusi explicit".green().bold(),             "→ List manually installed packages");
        println!("{} {}", "fusi foreign".green().bold(),              "→ Show packages from AUR");
        println!("{} {}", "fusi leaves".green().bold(),               "→ Show packages nothing depends on");
        println!("{} {}", "fusi files <pkg>".green().bold(),          "→ Show files owned by a package");
        println!("{} {}", "fusi size <pkg>".green().bold(),           "→ Show how much disk space a package uses");
        println!("{} {}", "fusi owner <file>".green().bold(),         "→ Show which package owns a file");
        println!("{} {}", "fusi deps <pkg>".green().bold(),           "→ Show dependencies of a package");
        println!("{} {}", "fusi verify <pkg>".green().bold(),         "→ Verify package files aren't corrupted");
        println!("{} {}", "fusi check".green().bold(),                "→ Check for broken dependencies");
        println!("{} {}", "fusi news".green().bold(),                 "→ Show latest Arch Linux news");
        println!("{} {}", "fusi history".green().bold(),              "→ Show last 20 pacman installs");
        println!("{} {}", "fusi log".green().bold(),                  "→ Show full pacman install history");
        println!("{} {}", "fusi stats".green().bold(),                "→ List all installed package names");
        println!("{} {}", "fusi cache".green().bold(),                "→ Show pacman cache size");
        println!("{} {}", "fusi cleancache".green().bold(),           "→ Clean old package cache");
        println!("{} {}", "fusi autoremove".green().bold(),           "→ Remove orphaned packages");
        println!("{} {}", "fusi backup".green().bold(),               "→ Backup installed packages to a file");
        println!("{} {}", "fusi restore".green().bold(),              "→ Restore packages from backup");
        println!("{} {}", "fusi mirrors".green().bold(),              "→ List your mirrorlist");
        println!("{} {}", "fusi unlock".green().bold(),               "→ Remove pacman lock file");
        println!("{} {}", "fusi self-update".green().bold(),          "→ Update fusi to the latest version");
        println!("{} {}", "fusi details".green().bold(),              "→ Show info about fusi");
        println!("{}", "─────────────────────────────────────────".cyan());
        println!("{}", "© 2026 fusiontech21 — AGPL-3.0".white());
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