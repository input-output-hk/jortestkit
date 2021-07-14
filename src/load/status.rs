use super::{
    progress::use_as_status_progress_bar,
    request::{Id, RequestStatus, Response},
    Monitor, RequestFailure,
};
use indicatif::ProgressBar;
use std::time::Duration;
use std::{
    sync::{
        mpsc::{self, Sender, TryRecvError},
        Arc, RwLock,
    },
    thread::{self, JoinHandle},
};

#[derive(Debug)]
pub struct StatusUpdaterThread {
    stop_signal: Sender<()>,
    handle: JoinHandle<()>,
}

#[derive(Debug, Clone)]
pub struct Status {
    duration: Duration,
    response_status: RequestStatus,
    id: Id,
}

impl Status {
    pub fn is_pending(&self) -> bool {
        match self.response_status {
            RequestStatus::Failed { .. } | RequestStatus::Success => false,
            RequestStatus::Pending => true,
        }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn status(&self) -> &RequestStatus {
        &self.response_status
    }

    pub fn duration(&self) -> &Duration {
        &self.duration
    }

    pub fn new_success(duration: Duration, id: Id) -> Self {
        Self {
            duration,
            response_status: RequestStatus::Success,
            id,
        }
    }

    pub fn new_pending(duration: Duration, id: Id) -> Self {
        Self {
            duration,
            response_status: RequestStatus::Pending,
            id,
        }
    }

    pub fn new_failure(duration: Duration, id: Id, messsage: String) -> Self {
        Self {
            duration,
            response_status: RequestStatus::Failed { message: messsage },
            id,
        }
    }
}
#[allow(clippy::from_over_into)]
impl Into<RequestStatus> for Status {
    fn into(self) -> RequestStatus {
        self.status().clone()
    }
}

impl Status {
    pub fn failure(&self) -> Option<RequestFailure> {
        match self.status() {
            RequestStatus::Pending | RequestStatus::Success => None,
            RequestStatus::Failed { message } => {
                Some(RequestFailure::Rejected(message.to_string()))
            }
        }
    }
}

pub trait RequestStatusProvider {
    fn get_statuses(&self, ids: &[Id]) -> Vec<Status>;
}

fn update_statuses(
    responses_clone: &Arc<RwLock<Vec<Response>>>,
    request_status_provider: &(impl RequestStatusProvider + Send),
) -> Vec<Status> {
    let mut responses = responses_clone.write().unwrap();
    let ids: Vec<Id> = responses
        .iter()
        .filter(|x| x.is_pending())
        .map(|resp| resp.id().as_ref().unwrap().clone())
        .collect();
    let statuses = request_status_provider.get_statuses(&ids);
    for status in statuses.iter() {
        for response in responses.iter_mut() {
            if response.has_id(status.id()) && !status.is_pending() {
                response.update_status(status.clone());
            }
        }
    }
    statuses
}

impl StatusUpdaterThread {
    pub fn spawn<S>(
        responses: &Arc<RwLock<Vec<Response>>>,
        request_status_provider: S,
        monitor: Monitor,
        title: &str,
        shutdown_grace_period: u32,
        pace: u64,
    ) -> Self
    where
        S: RequestStatusProvider + Send + 'static,
    {
        let (tx, rx) = mpsc::channel();
        let responses_clone = Arc::clone(&responses);
        let progress_bar = StatusProgressBar::new(
            ProgressBar::new(1),
            format!("[Load Scenario: {}]", title),
            monitor,
        );
        let monitor = thread::spawn(move || loop {
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    progress_bar.set_message("Waiting for all messages to be accepted or rejected");

                    for _ in 0..shutdown_grace_period {
                        let statuses = update_statuses(&responses_clone, &request_status_provider);
                        let pending_statuses: Vec<&Status> =
                            statuses.iter().filter(|x| x.is_pending()).collect();

                        if pending_statuses.is_empty() {
                            progress_bar.set_message("no pending messages");
                            return;
                        } else {
                            progress_bar.set_message(&format!(
                                "{} messages are still pending",
                                pending_statuses.len()
                            ));
                        }
                        std::thread::sleep(std::time::Duration::from_secs(1));
                    }
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }
            update_statuses(&responses_clone, &request_status_provider);
            std::thread::sleep(std::time::Duration::from_secs(pace));
        });
        Self {
            stop_signal: tx,
            handle: monitor,
        }
    }

    pub fn stop(self) {
        self.stop_signal.send(()).unwrap();
        self.handle.join().unwrap();
    }
}

struct StatusProgressBar {
    progress_bar: ProgressBar,
    prefix: String,
    monitor: Monitor,
}

impl StatusProgressBar {
    pub fn new(mut progress_bar: ProgressBar, prefix: String, monitor: Monitor) -> Self {
        use_as_status_progress_bar(&mut progress_bar);
        Self {
            progress_bar,
            prefix,
            monitor,
        }
    }

    pub fn set_message(&self, message: &str) {
        match &self.monitor {
            Monitor::Progress(..) => self
                .progress_bar
                .set_message(&format!("{} {}", self.prefix, message)),
            _ => println!("{} {}", self.prefix, message),
        }
    }
}
