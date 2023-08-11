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
            Halted,
        }
        let mut dayloop = DayLoop::new(State::Halted);
        let mut node: TimeLinkedListNode<State> = Local::now().time().into();
        node.on_enter(|s| *s = State::Halted);
        node.on_leave(|s| *s = State::Running);
        dayloop.insert(node);
        let s = dayloop.get();
        dayloop.get();
    }
}
