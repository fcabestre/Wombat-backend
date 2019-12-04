use std::fmt::format;
use std::str::{FromStr, SplitWhitespace};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialOrd, PartialEq)]
pub struct Cpu {
    name: String,
    user: u32,
    nice: u32,
    system: u32,
    idle: u32,
    iowait: u32,
    irq: u32,
    softirq: u32,
}

impl Cpu {
    fn convert(val: Option<&str>) -> Result<u32, String> {
        val.map_or(Result::Err("No value to convert".to_string()), |str_val| {
            u32::from_str(str_val).or(Result::Err(format(format_args!(
                "Cannot convert '{}' to integer",
                str_val.to_string()
            ))))
        })
    }

    pub fn parse(name: &str, mut stats_line: SplitWhitespace) -> Result<Cpu, String> {
        return Ok(Cpu {
            name: name.to_string(),
            user: Cpu::convert(stats_line.next())?,
            nice: Cpu::convert(stats_line.next())?,
            system: Cpu::convert(stats_line.next())?,
            idle: Cpu::convert(stats_line.next())?,
            iowait: Cpu::convert(stats_line.next())?,
            irq: Cpu::convert(stats_line.next())?,
            softirq: Cpu::convert(stats_line.next())?,
        });
    }
}

#[derive(Serialize, Deserialize, Debug, PartialOrd, PartialEq)]
pub struct Stats {
    cpus: Vec<Cpu>,
}

impl Stats {
    pub fn parse(stats_file_content: &str) -> Result<Stats, String> {
        let mut cpus: Vec<Cpu> = Vec::new();
        for stats_line in stats_file_content.lines() {
            let mut tokens = stats_line.split_whitespace();
            match tokens.next() {
                Some(cpu_name) if cpu_name.starts_with("cpu") => {
                    let cpu = Cpu::parse(cpu_name, tokens)?;
                    cpus.push(cpu)
                }
                _ => (),
            }
        }
        if cpus.is_empty() {
            Err("No CPU stats available".to_string())
        } else {
            Ok(Stats { cpus })
        }
    }
}

#[test]
fn test_cpu_convert() {
    assert_eq!(Cpu::convert(Some("4131912")), Ok(4131912u32));
    assert_eq!(Cpu::convert(None), Err("No value to convert".to_string()));
    assert_eq!(
        Cpu::convert(Some("Not a Number")),
        Err("Cannot convert 'Not a Number' to integer".to_string())
    );
}

#[test]
fn test_stats_parse_when_ok() {
    let sample = "
cpu  4131912 43485 887569 18935824 51604 0 102247 0 0 0
cpu0 960234 11659 231388 15668752 40519 0 35043 0 0 0
cpu1 916500 10916 214729 690046 3149 0 18904 0 0 0
cpu2 1480248 10422 249234 1296362 4548 0 45765 0 0 0
cpu3 774928 10486 192216 1280662 3386 0 2535 0 0 0
intr 7118640867 7 1171 0 0 0 0 0 0 1 43007 0 0 22939 0 0 0 585 2693749855 0 0 0 0 0 706 0 0 0 0 0 1865506 1344827 277 3363179 0 15 144 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
ctxt 678980518
btime 1572616833
processes 197673
procs_running 2
procs_blocked 0
softirq 89033313 5011 30024317 5135935 134668 1506402 84 7178334 27350448 0 17698114
    ";
    assert_eq!(
        Stats::parse(sample),
        Ok(Stats {
            cpus: vec![
                Cpu {
                    name: "cpu".to_string(),
                    user: 4131912,
                    nice: 43485,
                    system: 887569,
                    idle: 18935824,
                    iowait: 51604,
                    irq: 0,
                    softirq: 102247
                },
                Cpu {
                    name: "cpu0".to_string(),
                    user: 960234,
                    nice: 11659,
                    system: 231388,
                    idle: 15668752,
                    iowait: 40519,
                    irq: 0,
                    softirq: 35043
                },
                Cpu {
                    name: "cpu1".to_string(),
                    user: 916500,
                    nice: 10916,
                    system: 214729,
                    idle: 690046,
                    iowait: 3149,
                    irq: 0,
                    softirq: 18904
                },
                Cpu {
                    name: "cpu2".to_string(),
                    user: 1480248,
                    nice: 10422,
                    system: 249234,
                    idle: 1296362,
                    iowait: 4548,
                    irq: 0,
                    softirq: 45765
                },
                Cpu {
                    name: "cpu3".to_string(),
                    user: 774928,
                    nice: 10486,
                    system: 192216,
                    idle: 1280662,
                    iowait: 3386,
                    irq: 0,
                    softirq: 2535
                }
            ]
        })
    );
}

#[test]
fn test_stats_parse_when_missing_value() {
    let sample = "
cpu  4131912 43485 887569 18935824 51604 0
cpu0 960234 11659 231388 15668752 40519 0 35043 0 0 0
cpu1 916500 10916 214729 690046 3149 0 18904 0 0 0
cpu2 1480248 10422 249234 1296362 4548 0 45765 0 0 0
cpu3 774928 10486 192216 1280662 3386 0 2535 0 0 0
    ";
    assert_eq!(Stats::parse(sample), Err("No value to convert".to_string()));
}

#[test]
fn test_stats_parse_when_wrong_value() {
    let sample = "
cpu  4131912 oïoï 887569 18935824 51604 0 102247 0 0 0
cpu0 960234 11659 231388 15668752 40519 0 35043 0 0 0
cpu1 916500 10916 214729 690046 3149 0 18904 0 0 0
cpu2 1480248 10422 249234 1296362 4548 0 45765 0 0 0
cpu3 774928 10486 192216 1280662 3386 0 2535 0 0 0
    ";
    assert_eq!(
        Stats::parse(sample),
        Err("Cannot convert 'oïoï' to integer".to_string())
    );
}

#[test]
fn test_stats_parse_when_no_value() {
    let sample = "
    ";
    assert_eq!(
        Stats::parse(sample),
        Err("No CPU stats available".to_string())
    );
}
