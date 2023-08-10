mod time_linked_list;
pub use time_linked_list::*;


#[cfg(test)]
mod tests {
    use chrono::Local;

    use super::*;

    #[test]
    fn test_1() {
        pub enum State {
            Running,
            Halted
        }
        let mut list = DayLoop::new(State::Halted);
        list.insert(TimeLinkedListNode {
            start: Local::now().time(),
            enter: Some(|x: &mut State|*x = State::Running),
            leave: Some(|x: &mut State|*x = State::Halted),
        })
    }
}
