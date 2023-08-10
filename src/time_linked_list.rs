use chrono::{NaiveTime, Local};
use std::collections::{LinkedList, VecDeque};
pub struct TimeLinkedListNode<E, L> 
{
    pub start: NaiveTime,
    pub enter: Option<E>,
    pub leave: Option<L>
}


impl<E, L> From<NaiveTime> for TimeLinkedListNode<E, L>
{
    fn from(value: NaiveTime) -> Self {
        Self {
            start: value,
            enter: None,
            leave: None
        }
    }
}

pub struct DayLoop<T, E, L> {
    linked_list: VecDeque<TimeLinkedListNode<E, L>>,
    next_tick: NaiveTime,
    data: T
}

impl<T, E, L> DayLoop<T, E, L>
where E: Fn(&mut T), L: Fn(&mut T)
{
    pub fn new(data: T) -> Self {
        DayLoop {
            linked_list: VecDeque::new(),
            next_tick: Local::now().time(),
            data
        }
    }
    pub fn insert(&mut self, node: impl Into<TimeLinkedListNode<E, L>>) {
        let node = node.into();
        let pos = self.linked_list.iter().position(|x| x.start > node.start).map(|x| x - 1).unwrap_or(self.linked_list.len());
        self.linked_list.insert(pos, node);
    }

    pub fn get(&mut self) -> &mut T {
        if self.linked_list.len() <= 1 {
            return &mut self.data;
        }
        let now = Local::now().time();
        if now > self.next_tick {
            let tail = self.linked_list.back().unwrap();
            if let Some(f) = tail.leave.as_ref() { (f)(&mut self.data) }
            let front = self.linked_list.pop_front().unwrap();
            if let Some(f) = front.enter.as_ref() { (f)(&mut self.data) }
            self.next_tick = front.start;
            self.linked_list.push_back(front);
            return self.get()
        } else {
            &mut self.data
        }
    }
}