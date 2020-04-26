use chrono::{Local, TimeZone};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "epoch", about = "Given a Unix timestamp, gets the YMD HM")]
struct Opt {
    timestamps: Vec<i64>,
}

fn main() {
    let opt = Opt::from_args();
    if opt.timestamps.is_empty() {
        let timestamp = Local::now().timestamp();
        println!("{}", timestamp);
    } else {
        for timestamp in opt.timestamps.iter() {
            let datetime = Local::timestamp(&Local {}, *timestamp, 0);
            println!("{}:\t{}", timestamp, datetime.format("%T %F"));
        }
    }
}
