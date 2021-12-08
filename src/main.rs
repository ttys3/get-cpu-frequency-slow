use std::fs::File;
use std::io::Read;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    for i in 0..16 {
        get_cpu_frequency(i);
    }
    let duration = start.elapsed();
    println!("total time elapsed in get_cpu_frequency() is: {:?}", duration);
}

// the func `get_cpu_frequency` took from https://docs.rs/crate/sysinfo/0.19.2/source/src/linux/processor.rs
// mainly for debugging and verify that delta 0.10.x slowdown problem is caused by `sysinfo` crate
// see https://github.com/dandavison/delta/issues/839
fn get_cpu_frequency(cpu_core_index: usize) -> u64 {
    let mut s = String::new();
    if File::open(format!(
        "/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq",
        cpu_core_index
    ))
    .and_then(|mut f| f.read_to_string(&mut s))
    .is_ok()
    {
        let freq_option = s.trim().split('\n').next();
        if let Some(freq_string) = freq_option {
            if let Ok(freq) = freq_string.parse::<u64>() {
                return freq / 1000;
            }
        }
    }
    s.clear();

    println!("try get cpu freq from sysfs failed, fallback to /proc/cpuinfo");

    if File::open("/proc/cpuinfo")
        .and_then(|mut f| f.read_to_string(&mut s))
        .is_err()
    {
        return 0;
    }
    let find_cpu_mhz = s.split('\n').find(|line| {
        line.starts_with("cpu MHz\t")
            || line.starts_with("BogoMIPS")
            || line.starts_with("clock\t")
            || line.starts_with("bogomips per cpu")
    });
    find_cpu_mhz
        .and_then(|line| line.split(':').last())
        .and_then(|val| val.replace("MHz", "").trim().parse::<f64>().ok())
        .map(|speed| speed as u64)
        .unwrap_or_default()
}
