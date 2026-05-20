# Social Engineering Awareness Tool (Rust)

A beginner-friendly cybersecurity awareness toolkit written in Rust.

This project demonstrates basic concepts related to:

- Phishing detection
- Suspicious link analysis
- Password security
- Social engineering awareness

⚠️ Educational use only.

---

# Features

## Phishing Email Detector

Checks email text for suspicious keywords such as:

- urgent
- verify
- password
- login now
- free money
- account suspended

---

## Suspicious Link Analyzer

Detects risky URL patterns such as:

```text
bit.ly
tinyurl
@
verify
secure-account
```

---

## Password Strength Checker

Checks for:

- Minimum length
- Uppercase letters
- Lowercase letters
- Numbers
- Special characters

---

## Awareness Tips

Displays cybersecurity awareness guidance.

---

# Requirements

Install Rust:

https://www.rust-lang.org/tools/install

Verify installation:

```bash
rustc --version
cargo --version
```

---

# Project Structure

```text
Social-Engineering-Awareness/
│
├── src/
│   └── main.rs
│
├── Cargo.toml
├── README.md
```

---

# How To Create Project

```bash
cargo new Social-Engineering-Awareness
cd Social-Engineering-Awareness
```

Replace:

```text
src/main.rs
```

with the provided code.

---

# How To Run

```bash
cargo run
```

---

# How To Build

```bash
cargo build --release
```

Built executable:

```text
target/release/Social-Engineering-Awareness
```

---

# Menu

```text
====================================
 Social Engineering Awareness Tool
====================================
1. Analyze Email
2. Analyze Link
3. Password Strength Checker
4. Awareness Tips
5. Exit
====================================
```

---

# Example Usage

## Analyze Email

Input:

```text
URGENT: Verify your account now or it will be suspended.
```

Output:

```text
[WARNING] Suspicious keyword detected: urgent
[WARNING] Suspicious keyword detected: verify
```

---

## Analyze Link

Input:

```text
https://bit.ly/free-login
```

Output:

```text
[WARNING] Suspicious pattern found: bit.ly
[WARNING] Suspicious pattern found: login
```

---

# Learning Goals

This project helps beginners understand:

- Social engineering tactics
- Phishing awareness
- Password security
- Cybersecurity best practices
- Defensive security concepts

---

# Recommended Tools To Learn

- GoPhish
- Wireshark
- Burp Suite
- Kali Linux
- Have I Been Pwned

---

# Disclaimer

This project is intended for:

- Education
- Cybersecurity awareness
- Training environments

Do NOT use for phishing, impersonation, or malicious activity.

---

# License

MIT License
