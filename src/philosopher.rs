use crate::fork::Fork;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::sleep;

#[derive(Clone)]
pub struct Philosopher {
    pub id: u32,
    pub left_fork: Arc<Mutex<Fork>>,
    pub right_fork: Arc<Mutex<Fork>>,
    pub time_eating: u128,
    pub time_waiting: u128,
    pub time_thinking: u128,
}

impl Philosopher {
    pub fn new(id: u32, left_fork: Arc<Mutex<Fork>>, right_fork: Arc<Mutex<Fork>>) -> Philosopher {
        Philosopher {
            id,
            left_fork,
            right_fork,
            time_eating: 0,
            time_waiting: 0,
            time_thinking: 0,
        }
    }

    pub async fn eat(&mut self) {
        // random time to eat
        let time = rand::random::<u64>() % 1000;
        let starting_time = std::time::SystemTime::now();

        println!("Philosopher {} is hungry!", self.id);

        let mut left_fork = self.left_fork.lock().await;

        left_fork.take();

        let mut right_fork = self.right_fork.lock().await;

        right_fork.take();

        let time_waiting = starting_time.elapsed().unwrap().as_millis();

        self.time_waiting += time_waiting;

        println!("Philosopher {} is eating for {} ms!", self.id, time);

        self.time_eating += u128::from(time);

        sleep(std::time::Duration::from_millis(time)).await;

        left_fork.put();
        right_fork.put();
    }

    pub async fn think(&mut self) {
        // random time to think
        let time = rand::random::<u64>() % 1000;

        println!("Philosopher {} is thinking for {} ms!", self.id, time);

        self.time_thinking += u128::from(time);

        sleep(std::time::Duration::from_millis(time)).await;
    }
}
