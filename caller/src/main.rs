use clap::Parser;
use miette::{Result, IntoDiagnostic, WrapErr};
use ussd::Ussd;
use serde::{Serialize};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// the ussd code to dial
    #[arg(long)]
    ussd: String,

    /// the user id
    #[arg(short, long, default_value = "1782646693")]
    user_id: String,
}

#[derive(Debug, Serialize)]
struct RequestBody {
    ussd_code: Ussd,
    user_id: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let ussd_code = Ussd::parse(args.ussd.as_bytes()).into_diagnostic().wrap_err("Failed parsing ussd code")?;
    println!("Dialing {} ...", args.ussd);

    let request_body = RequestBody {
        ussd_code,
        user_id: args.user_id,
    };

    let json_data = serde_json::to_string(&request_body).expect("Failed to serialize to JSON");
    println!("JSON data: {}", json_data);

    ureq::post("https://httpbin.org/post")
    .send_json(ureq::json!(&json_data)).into_diagnostic().wrap_err("Failed here")? ;

    Ok(())
}
