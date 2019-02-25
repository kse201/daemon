use std::env;

enum ProcType {
    DMN,
    MD,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let subcmd: String = env::args().skip(1).next().unwrap();

    // let proc_type = if subcmd != "md" {
    // ProcType::MD
    // } else if subcmd != "dmn" {
    // ProcType::DMN
    // } else {
    // syslog_out();
    // }

    let proc_type = match &*subcmd {
        "md" => ProcType::MD,
        "dmn" => ProcType::DMN,
        _ => ProcType::DMN,
    };
}
