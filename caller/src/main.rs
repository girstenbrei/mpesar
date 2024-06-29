use clap::Parser;
use miette::{IntoDiagnostic, Result, WrapErr};
use serde::Serialize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// the ussd code to dial
    #[arg(long)]
    ussd: String,

    /// the user id
    #[arg(short, long, default_value = "1782646693")]
    user_id: i32,

    /// The system endpoint
    #[arg(short, long, default_value = "http://localhost:3000")]
    endpoint: String,
}

#[derive(Debug, Serialize)]
struct RequestBody {
    ussd_code: String,
    account_id: i32,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // let ussd_code = Ussd::parse(args.ussd.as_bytes()).into_diagnostic().wrap_err("Failed parsing ussd code")?;
    println!("Dialing {} ...", args.ussd);

    let request_body = RequestBody {
        ussd_code: args.ussd,
        account_id: args.user_id,
    };

    ureq::post(&args.endpoint)
        .send_json(ureq::json!(&request_body))
        .into_diagnostic()
        .wrap_err("Failed here")?;

    Ok(())
}
