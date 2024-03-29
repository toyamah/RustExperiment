use std::cell::RefCell;
use std::rc::Rc;

pub fn main() {
    utils::println_file_name!();
    multiple_owners_of_mutable_data();
}

fn immutable_data_cannot_borrow_as_mutable() {
    // let x = 1;
    // let y = &mut x;
    //
    // 6 |     let x = 1;
    //   |         - help: consider changing this to be mutable: `mut x`
    // 7 |     let y = &mut x;
    //   |             ^^^^^^ cannot borrow as mutable
}

/// A Use Case for Interior Mutability: Mock Objects
/// https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#a-use-case-for-interior-mutability-mock-objects
mod mock {
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

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::cell::{RefCell, RefMut};

        struct MockMessenger {
            sent_messages: Vec<String>,
        }

        impl Messenger for MockMessenger {
            fn send(&self, msg: &str) {
                // cannot compile because Messenger trait requires immutable self reference.
                // This situation can be solve using RefCell.
                // self.sent_messages.push(msg.to_string())
            }
        }

        struct MockMessenger2 {
            sent_messages: RefCell<Vec<String>>,
        }

        impl Messenger for MockMessenger2 {
            fn send(&self, msg: &str) {
                // RefCell enables the hold immutable reference to borrow as mutable.
                self.sent_messages.borrow_mut().push(msg.to_string());
            }
        }

        #[test]
        fn it_sends_an_over_75_percent_warning_message() {
            let mock_messenger = MockMessenger2 {
                sent_messages: RefCell::new(vec![]),
            };
            let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

            limit_tracker.set_value(80);

            assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        }

        /// https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#keeping-track-of-borrows-at-runtime-with-refcellt
        struct MockMessenger3 {
            sent_messages: RefCell<Vec<String>>,
        }

        impl Messenger for MockMessenger3 {
            fn send(&self, msg: &str) {
                let mut ref_mut1: RefMut<_> = self.sent_messages.borrow_mut();
                // the below will panic because sent_message already borrows a mutable reference.
                let mut ref_mut2: RefMut<_> = self.sent_messages.borrow_mut();
            }
        }

        /// should panic instead of compile error
        /// because Rust checks Borrowing rules at runtime if using RefMut.
        #[test]
        #[should_panic]
        fn panic_if_multiple_mutable_references() {
            let mock_messenger = MockMessenger3 {
                sent_messages: RefCell::new(vec![]),
            };
            let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
            limit_tracker.set_value(80);
        }
    }
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

/// Using Rc and RefCell can have multiple owners of mutable data.
fn multiple_owners_of_mutable_data() {
    use self::List::{Cons, Nil};
    utils::println_function_name!();

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
