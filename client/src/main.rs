use anyhow::Result;
use std::io::{BufRead, BufReader};

fn rest_call(req: &str) -> Result<()> {
    let mut res = ureq::get(format!("http://localhost:3000/{}", req)).call()?;
    let status = res.status();
    if status != ureq::http::StatusCode::OK {
        let json_str: serde_json::Value = serde_json::json!({
            "status": serde_json::Value::String(status.to_string()),
        });
        let json_str = serde_json::to_string_pretty(&json_str)?;
        println!("{}", json_str);
        return Ok(());
    }
    let content_type = res
        .headers()
        .get("content-type")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("")
        .to_lowercase();
    let body = res.body_mut();
    if content_type.contains("application/x-ndjson") {
        let reader = BufReader::new(body.as_reader());
        for line in reader.lines() {
            let line = line?;
            if line.is_empty() {
                continue;
            }
            let json_val: serde_json::Value = serde_json::from_str(&line)?;
            let json_str = serde_json::to_string_pretty(&json_val)?;
            println!("{}", json_str);
        }
    } else if content_type.contains("application/json") {
        let body = body.read_json::<serde_json::Value>()?;
        let json_str = serde_json::to_string_pretty(&body)?;
        println!("{}", json_str);
    } else {
        let body = body.read_to_string()?;
        println!("{}", body);
    }
    Ok(())
}

fn main() -> Result<()> {
    println!("-[json]-----------------------");
    rest_call("json")?;
    println!("-[stream]-----------------------");
    rest_call("stream")?;
    Ok(())
}
