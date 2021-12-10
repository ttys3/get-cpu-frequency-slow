# get-cpu-frequency-slow

the func `get_cpu_frequency` took from https://docs.rs/crate/sysinfo/0.19.2/source/src/linux/processor.rs
mainly for debugging and verify that delta 0.10.x slowdown problem is caused by `sysinfo` crate
see https://github.com/dandavison/delta/issues/839

## usage

```shell
get-cpu-frequency-slow [your_machine_logic_CPU_num]
```

## demo result

test env:

```
------------
OS: Arch Linux x86_64
Kernel: 5.15.6-arch2-1
Shell: zsh 5.8
Resolution: 3840x2160
Terminal: tmux
CPU: Intel i9-9900 (16) @ 5.000GHz
GPU: NVIDIA GeForce GTX 1060 3GB
Memory: 6747MiB / 32038MiB
```

a single run will cost about `16ms`, thus 16 core CPU result in about `256ms`

```
./target/release/get-cpu-frequency-slow 1
num_cpu=1
total time elapsed in get_cpu_frequency()x1 is: 15.934353ms
./target/release/get-cpu-frequency-slow
num_cpu=16
total time elapsed in get_cpu_frequency()x16 is: 252.667394ms
```


most of the time it will look like this:

```shell
./target/release/get-cpu-frequency-slow
total time elapsed in get_cpu_frequency() is: 264.490103ms

❯ make run
./target/release/get-cpu-frequency-slow
total time elapsed in get_cpu_frequency() is: 264.542344ms

❯ make run
./target/release/get-cpu-frequency-slow
total time elapsed in get_cpu_frequency() is: 265.281557ms

❯ make run
./target/release/get-cpu-frequency-slow
total time elapsed in get_cpu_frequency() is: 264.696121ms
```

but there's also **rarely** result in very low latency result, like this:

```
❯ make run
./target/release/get-cpu-frequency-slow
total time elapsed in get_cpu_frequency() is: 275.174µs
```

## Important notice

please note that directly read /proc/cpuinfo is rather fast.

this program is only used to demonstrate the slowdown problem when reading `/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq`

```shell
# with read /sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq
❯ ./target/release/get-cpu-frequency-slow 16
num_cpu=16 force_cpuinfo=false
total time elapsed in get_cpu_frequency()x16 is: 281.183772ms

# directly read /proc/cpuinfo
❯ ./target/release/get-cpu-frequency-slow 16 true
num_cpu=16 force_cpuinfo=true
skip readding cpu freq from sysfs, directly to /proc/cpuinfo
total time elapsed in get_cpu_frequency()x16 is: 2.989249ms
```