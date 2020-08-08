/**
 * A library that tracks a value against a maximum value and
 * sends messages based on how close to the maximum value the current value .
 *
 * We will try to implement RefCell with Test case that mocks Messenger object.
 */

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    //Testing this function by testing if the messages being sent are correct
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Error: You have used up over 90% of your quota");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    //Create a mock struct that will implement Messenger
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    //Implements Messenger but instead of sending message, it stores it in property sent_messages for comparision later
    //self is immutable reference but we want to push messages to it. We cannot make it mutable or we will not be implementing Messenger Trait
    // RefCell is used to fix that.
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            //RefCell method borrow_mut  gives mutable reference -> RefMut<T>
            let mut borrow = self.sent_messages.borrow_mut();
            // let mut borrow2 = self.sent_messages.borrow_mut(); This line panics at runtime. already borrowed:BorrowMutError
            borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        //immutable reference -> Ref<T>
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
