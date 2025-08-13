
#[derive(Clone, Copy, Debug)]
pub enum Lane { P0, P1, P2 }
#[derive(Clone, Copy, Debug, Default)]
pub struct Budget { pub ns: u64 }
#[derive(Clone, Debug)]
pub struct Job { pub lane: Lane, pub deadline_ns: u64, pub id: u64 }
#[derive(Default)]
pub struct PriQueue { jobs: Vec<Job> }
impl PriQueue {
    pub fn schedule(&mut self, job: Job) { self.jobs.push(job); }
    pub fn poll_ready(&mut self, _budget: Budget) -> Vec<Job> {
        let mut out = Vec::new(); std::mem::swap(&mut out, &mut self.jobs); out
    }
}
pub struct Budgets { pub idle: Budget, pub observe: Budget, pub focus: Budget, pub react: Budget }
impl Default for Budgets {
    fn default() -> Self {
        Self {
            idle: Budget { ns: 1_000_000 },
            observe: Budget { ns: 5_000_000 },
            focus: Budget { ns: 5_000_000 },
            react: Budget { ns: 2_000_000 },
        }
    }
}
impl Budgets {
    pub fn for_mode(&self, m: crate::model::Mode) -> Budget {
        match m {
            crate::model::Mode::Idle => self.idle,
            crate::model::Mode::Observe => self.observe,
            crate::model::Mode::Focus => self.focus,
            crate::model::Mode::React => self.react,
        }
    }
}
