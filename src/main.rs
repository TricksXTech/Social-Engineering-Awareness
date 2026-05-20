// ==============================================
// Educational Social Engineering Awareness Tool
// Language: Rust
// ==============================================
// Features:
// - Phishing Email Detector
// - Suspicious Link Analyzer
// - Password Strength Checker
// - Awareness Training
//
// Run:
// cargo run
//
// Build:
// cargo build --release
//
// Educational Use Only
// ==============================================

use std::io;

// ==============================================
// PHISHING DETECTOR
// ==============================================

fn phishing_detector(email: &str) {
    let suspicious_keywords = vec![
        "urgent",
        "verify",
        "password",
        "bank",
        "click here",
        "limited time",
        "login now",
        "free money",
        "account suspended",
    ];

    println!("\n[+] Phishing Analysis\n");

    let email_lower = email.to_lowercase();

    let mut detected = false;

    for keyword in suspicious_keywords {
        if email_lower.contains(keyword) {
            println!("[WARNING] Suspicious keyword detected: {}", keyword);
            detected = true;
        }
    }

    if !detected {
        println!("No suspicious keywords detected.");
    }
}

// ==============================================
// LINK ANALYZER
// ==============================================

fn link_analyzer(link: &str) {
    println!("\n[+] Link Analysis\n");

    let suspicious_patterns = vec![
        "@",
        "bit.ly",
        "tinyurl",
        "free",
        "login",
        "verify",
        "secure-account",
    ];

    let lower = link.to_lowercase();

    let mut flagged = false;

    for pattern in suspicious_patterns {
        if lower.contains(pattern) {
            println!("[WARNING] Suspicious pattern found: {}", pattern);
            flagged = true;
        }
    }

    if !flagged {
        println!("No suspicious patterns detected.");
    }
}

// ==============================================
// PASSWORD STRENGTH CHECKER
// ==============================================

fn password_strength(password: &str) {
    println!("\n[+] Password Strength Check\n");

    let length = password.len();

    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_number = password.chars().any(|c| c.is_numeric());
    let has_special = password.chars().any(|c| !c.is_alphanumeric());

    if length >= 12 && has_upper && has_lower && has_number && has_special {
        println!("Strong Password");
    } else {
        println!("Weak Password");

        if length < 12 {
            println!("- Use at least 12 characters");
        }

        if !has_upper {
            println!("- Add uppercase letters");
        }

        if !has_lower {
            println!("- Add lowercase letters");
        }

        if !has_number {
            println!("- Add numbers");
        }

        if !has_special {
            println!("- Add special characters");
        }
    }
}

// ==============================================
// AWARENESS TIPS
// ==============================================

fn awareness_tips() {
    println!(
        "
====================================
 Social Engineering Awareness Tips
====================================

1. Never share passwords
2. Verify suspicious emails
3. Check URLs carefully
4. Avoid unknown attachments
5. Enable 2FA
6. Do not trust urgency tactics
7. Verify identities independently
8. Avoid public Wi-Fi for sensitive work

====================================
"
    );
}

// ==============================================
// INPUT HELPER
// ==============================================

fn get_input(prompt: &str) -> String {
    let mut input = String::new();

    println!("{}", prompt);

    io::stdin()
        .read_line(&mut input)
        .expect(\"Failed to read input\");

    input.trim().to_string()
}

// ==============================================
// MAIN MENU
// ==============================================

fn main() {
    loop {
        println!(
            "
====================================
 Social Engineering Awareness Tool
====================================
1. Analyze Email
2. Analyze Link
3. Password Strength Checker
4. Awareness Tips
5. Exit
====================================
"
        );

        let choice = get_input("Select Option:");

        match choice.as_str() {
            "1" => {
                let email = get_input("Paste Email Content:");
                phishing_detector(&email);
            }

            "2" => {
                let link = get_input("Enter URL:");
                link_analyzer(&link);
            }

            "3" => {
                let password = get_input("Enter Password:");
                password_strength(&password);
            }

            "4" => {
                awareness_tips();
            }

            "5" => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Invalid Option");
            }
        }
    }
}
