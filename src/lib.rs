extern crate libc;

use libc::*;
//use std::collections::{HashSet};
use std::fs::{File};
use std::io::{BufRead, BufReader};
use std::mem::{size_of_val, uninitialized};
use std::path::{PathBuf};
//use std::process::{Command};

/*pub enum CpuLevel {
  Processor,
  Core,
  Thread,
}*/

#[cfg(target_os = "linux")]
pub fn set_affinity(thr_idxs: &[usize]) -> Result<(), ()> {
  unsafe {
    let mut cpuset = uninitialized();
    CPU_ZERO(&mut cpuset);
    for &thr_idx in thr_idxs {
      CPU_SET(thr_idx, &mut cpuset);
    }
    let thread = pthread_self();
    match pthread_setaffinity_np(thread, size_of_val(&cpuset), &cpuset as *const _) {
      0 => Ok(()),
      _ => Err(()),
    }
  }
}

pub enum CpuTopologySource {
  Auto,
  LinuxProcCpuinfo,
}

#[derive(Debug)]
pub struct CpuThreadInfo {
  pub thr_idx:  usize,
  pub core_idx: usize,
  pub proc_idx: usize,
}

#[derive(Debug)]
pub struct CpuTopology {
  pub threads:      Vec<CpuThreadInfo>,
}

impl CpuTopology {
  pub fn query(source: CpuTopologySource) -> CpuTopology {
    match source {
      CpuTopologySource::Auto => {
        unimplemented!();
      }
      CpuTopologySource::LinuxProcCpuinfo => {
        Self::query_proc_cpuinfo()
      }
    }
  }

  fn query_proc_cpuinfo() -> CpuTopology {
    let file = File::open(&PathBuf::from("/proc/cpuinfo"))
      .unwrap();
    let reader = BufReader::new(file);

    //let mut thread_set = HashSet::new();
    let mut threads = vec![];
    let mut curr_thread = None;
    for line in reader.lines() {
      let line = line.unwrap();
      if line.len() >= 9 && &line[ .. 9] == "processor" {
        let toks: Vec<_> = line.splitn(2, ":").collect();

        // Not assuming processor numbers are consecutive.
        let thread_idx: usize = toks[1].trim().parse().unwrap();
        //thread_set.insert(thread_idx);

        if let Some(info) = curr_thread {
          threads.push(info);
        }

        curr_thread = Some(CpuThreadInfo{
          thr_idx:  thread_idx,
          core_idx: 0,
          proc_idx: 0,
        });

      } else if line.len() >= 7 && &line[ .. 7] == "core id" {
        let toks: Vec<_> = line.splitn(2, ":").collect();

        let core_idx: usize = toks[1].trim().parse().unwrap();
        if let Some(ref mut info) = curr_thread {
          info.core_idx = core_idx;
        }

      } else if line.len() >= 11 && &line[ .. 11] == "physical id" {
        let toks: Vec<_> = line.splitn(2, ":").collect();

        let proc_idx: usize = toks[1].trim().parse().unwrap();
        if let Some(ref mut info) = curr_thread {
          info.proc_idx = proc_idx;
        }

      }
    }
    if let Some(info) = curr_thread {
      threads.push(info);
    }
    //let num_threads = processor_set.len();

    CpuTopology{
      threads:      threads,
    }
  }

  pub fn num_threads(&self) -> usize {
    self.threads.len()
  }
}
