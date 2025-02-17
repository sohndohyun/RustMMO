pub mod mpsc {
    use std::collections::VecDeque;
    use std::rc::{Rc, Weak};
    use std::cell::RefCell;

    pub struct Sender<T> {
        queue: Rc<RefCell<VecDeque<T>>>,
    }

    impl<T> Sender<T> {
        pub fn send(&self, value: T) {
            self.queue.borrow_mut().push_back(value);
        }

        pub fn is_disconnected(&self) -> bool {
            Rc::strong_count(&self.queue) == 1
        }

        pub fn clone(&self) -> Self {
            Sender {queue: self.queue.clone() }
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    pub enum ReceiveError {
        /// Queue is empty
        Empty,
        /// No queue available (disconnected)
        Disconnected,
    }

    pub struct Receiver<T> {
        queue: Weak<RefCell<VecDeque<T>>>,
    }

    impl<T> Receiver<T> {
        pub fn try_receive(&self) -> Result<T, ReceiveError> {
            if let Some(queue) = self.queue.upgrade() {
                let mut queue_ref = queue.borrow_mut();
                if let Some(value) = queue_ref.pop_front() {
                    Ok(value)
                } else if Rc::strong_count(&queue) == 1 {
                    Err(ReceiveError::Disconnected)
                } else {
                    Err(ReceiveError::Empty)
                }
            } else {
                Err(ReceiveError::Disconnected)
            }
        }
    }

    pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
        let queue = Rc::new(RefCell::new(VecDeque::new()));
        (Sender { queue: queue.clone() }, Receiver { queue: Rc::downgrade(&queue) })
    }
}