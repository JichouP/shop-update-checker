use clokwerk::{Interval, Scheduler};

pub fn cron<F>(f: F)
where
    F: Fn() + Send + 'static,
{
    let mut scheduler = Scheduler::new();
    scheduler.every(Interval::Hours(1)).run(f);

    loop {
        scheduler.run_pending()
    }
}
