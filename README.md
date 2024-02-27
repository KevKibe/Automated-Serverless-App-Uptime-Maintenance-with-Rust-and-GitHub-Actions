## Description

This project is a Rust script and a GitHub Actions workflow to maintain the uptime of a serverless application, to prevent spinning down during periods of inactivity, on free-tier platforms.<br>
This is done by executing a GET request every 5 mins using the [reqwests crate](https://docs.rs/reqwest/latest/reqwest/).

## Usage
1. Clone the repository.
```bash
git clone
```
2. Replace the existing URLs in the urls vector with the actual URLs of your serverless applications 
