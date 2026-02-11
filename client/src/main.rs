use anyhow::Result;
use std::io::{BufRead, BufReader};

fn json_call(req: &str) -> Result<()> {
    let mut res = ureq::get(format!("http://localhost:3000/{}", req)).call()?;
    let status = res.status();
    let body = res.body_mut().read_json::<serde_json::Value>()?;

    if status == ureq::http::StatusCode::OK {
        let json_str = serde_json::to_string_pretty(&body)?;
        println!("{}", json_str);
    } else {
        let json_str: serde_json::Value = serde_json::json!({
            "status": serde_json::Value::String(status.to_string()),
            "error": body,
        });
        let json_str = serde_json::to_string_pretty(&json_str)?;
        println!("{}", json_str);
    }
    Ok(())
}

fn stream_call(req: &str) -> Result<()> {
    let mut res = ureq::get(format!("http://localhost:3000/{}", req)).call()?;
    let status = res.status();

    if status == ureq::http::StatusCode::OK {
        let reader = BufReader::new(res.body_mut().as_reader());
        for line in reader.lines() {
            let line = line?;
            if line.is_empty() { continue; }

            let json_val: serde_json::Value = serde_json::from_str(&line)?;
            let json_str = serde_json::to_string_pretty(&json_val)?;
            println!("{}", json_str);
        }
    } else {
        let json_str: serde_json::Value = serde_json::json!({
            "status": serde_json::Value::String(status.to_string()),
        });
        let json_str = serde_json::to_string_pretty(&json_str)?;
        println!("{}", json_str);
    }
    Ok(())
}

fn main() -> Result<()> {
    println!("-[json]-----------------------");
    json_call("json")?;
    println!("-[stream]-----------------------");
    stream_call("stream")?;
    Ok(())
}
