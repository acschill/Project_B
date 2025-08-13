
use crate::analysis::{estimate_salience, update_scene};
use crate::config::{Budgets, Cadence, ModeThresholds};
use crate::interfaces::{Inbound, Outbound, IMHealth};
use crate::model::{Mode, WorkingContext};
use crate::scheduler::{PriQueue, Job};
use crate::storage::{WAL, Snapshot};
use crate::health::HealthStatus;
use anyhow::Result;
use std::time::Instant;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::time::sleep;

pub struct ImController {
    pub ctx: WorkingContext,
    pub priq: PriQueue,
    pub budgets: Budgets,
    pub cadence: Cadence,
    pub thresholds: ModeThresholds,
    pub rx_in: UnboundedReceiver<Inbound>,
    pub tx_out: UnboundedSender<Outbound>,
    pub wal: WAL,
    pub snapshot: Snapshot,
    last_tick: Instant,
}

impl ImController {
    pub fn new(
        rx_in: UnboundedReceiver<Inbound>,
        tx_out: UnboundedSender<Outbound>,
        wal: WAL,
        snapshot: Snapshot,
    ) -> Self {
        Self {
            ctx: WorkingContext::default(),
            priq: PriQueue::default(),
            budgets: Budgets::default(),
            cadence: Cadence::default(),
            thresholds: ModeThresholds::default(),
            rx_in,
            tx_out,
            wal,
            snapshot,
            last_tick: Instant::now(),
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            self.tick()?;
            while let Ok(ev) = self.rx_in.try_recv() {
                self.on_event(ev)?;
            }
            self.persist()?;
            self.health_checks()?;
            sleep(self.cadence.sleep_for(self.ctx.mode)).await;
        }
    }

    pub fn tick(&mut self) -> Result<()> {
        let s = estimate_salience(&self.ctx);
        self.ctx.apply_salience(s, &self.thresholds);
        let _jobs: Vec<Job> = self.priq.poll_ready(self.budgets.for_mode(self.ctx.mode));
        let _ = self.tx_out.send(Outbound::Health(IMHealth::from(&self.ctx)));
        Ok(())
    }

    pub fn on_event(&mut self, ev: Inbound) -> Result<()> {
        match ev {
            Inbound::AEIUpdate(a) => { update_scene(&mut self.ctx, &[a.into_event()]); }
            _ => {}
        }
        Ok(())
    }

    pub fn persist(&mut self) -> Result<()> {
        self.wal.maybe_flush()?;
        if self.cadence.should_snapshot(self.ctx.mode) {
            self.snapshot.write_snapshot(&self.ctx)?;
        }
        Ok(())
    }

    pub fn apply_backpressure(&mut self) -> Result<()> { Ok(()) }

    pub fn health_checks(&mut self) -> Result<HealthStatus> { Ok(HealthStatus::Ok) }
}
