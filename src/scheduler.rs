// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Job-Scheduler mit Resource-Slots (ATC-STD-500..509 DRAFTs, MVP).

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Job {
    pub id: u64,
    pub slots: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedError {
    QueueEmpty,
    UnknownJob,
}

pub struct Scheduler {
    max_slots: u32,
    free_slots: u32,
    queue: Vec<Job>,
    running: Vec<Job>,
}

impl Scheduler {
    pub fn new(max_slots: u32) -> Self {
        Scheduler { max_slots, free_slots: max_slots, queue: Vec::new(), running: Vec::new() }
    }

    pub fn enqueue(&mut self, job: Job) {
        self.queue.push(job);
    }

    /// Startet FIFO-Jobs, solange Slots frei sind.
    pub fn dispatch(&mut self) -> Vec<Job> {
        let mut started = Vec::new();
        while let Some(job) = self.queue.first() {
            if job.slots > self.free_slots {
                break;
            }
            let job = self.queue.remove(0);
            self.free_slots -= job.slots;
            self.running.push(job.clone());
            started.push(job);
        }
        started
    }

    pub fn finish(&mut self, id: u64) -> Result<(), SchedError> {
        match self.running.iter().position(|j| j.id == id) {
            Some(pos) => {
                let job = self.running.remove(pos);
                self.free_slots += job.slots;
                Ok(())
            }
            None => Err(SchedError::UnknownJob),
        }
    }

    pub fn free_slots(&self) -> u32 {
        self.free_slots
    }

    pub fn max_slots(&self) -> u32 {
        self.max_slots
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slot_limit_und_fifo() {
        let mut s = Scheduler::new(4);
        s.enqueue(Job { id: 1, slots: 2 });
        s.enqueue(Job { id: 2, slots: 2 });
        s.enqueue(Job { id: 3, slots: 1 });
        let started = s.dispatch();
        assert_eq!(started.len(), 2);
        assert_eq!(s.free_slots(), 0);
        assert_eq!(s.finish(1), Ok(()));
        assert_eq!(s.free_slots(), 2);
        let started = s.dispatch();
        assert_eq!(started.len(), 1);
    }

    #[test]
    fn unbekannter_job() {
        let mut s = Scheduler::new(2);
        assert_eq!(s.finish(9), Err(SchedError::UnknownJob));
    }
}
