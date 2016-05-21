extern crate cpu_topo;

use cpu_topo::{CpuTopologySource, CpuTopology};

fn main() {
  let topo = CpuTopology::query(CpuTopologySource::LinuxProcCpuinfo);
  println!("num threads: {}", topo.num_threads());
  println!("threads:     {:?}", topo.threads);
}
