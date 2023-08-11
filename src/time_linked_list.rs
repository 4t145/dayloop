use chrono::{Local, NaiveTime};
use std::fmt::Debug;
use std::{
    collections::{LinkedList, VecDeque},
    ops::Deref,
    sync::{Arc, RwLock},
};
pub struct TimeLinkedListNode<T> {
    pub start: NaiveTime,
    pub enter: Box<dyn Fn(&mut T)>,
    pub leave: Box<dyn Fn(&mut T)>,
}

impl<T: 'static> TimeLinkedListNode<T> {
    pub fn new(time: NaiveTime) -> Self {
        time.into()
    }
    pub fn on_enter(&mut self, mapper: impl Fn(&mut T) + 'static) {
        self.enter = Box::new(mapper)
    }
    pub fn on_leave(&mut self, mapper: impl Fn(&mut T) + 'static) {
        self.leave = Box::new(mapper)
    }
}

fn identity<T>(_: &mut T) {}

impl<T: 'static> From<NaiveTime> for TimeLinkedListNode<T> {
    fn from(value: NaiveTime) -> Self {
        Self {
            start: value,
            enter: Box::new(identity),
            leave: Box::new(identity),
        }
    }
}

impl<T: Debug> Debug for TimeLinkedListNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TimeLinkedListNode")
            .field("start", &self.start)
            .finish()
    }
}

#[derive(Debug)]
pub struct DayLoop<T> {
    blocks: VecDeque<TimeLinkedListNode<T>>,
    current_block: usize,
    next_tick: NaiveTime,
    data: T,
}

impl<T> DayLoop<T> {
    pub fn new(data: T) -> Self {
        DayLoop {
            blocks: VecDeque::from([
                TimeLinkedListNode::new(NaiveTime::MIN)
            ]),
            current_block: 0,
            next_tick: Local::now().time(),
            data,
        }
    }
    pub fn next_block(&self) {

    }
    pub fn insert(&mut self, node: impl Into<TimeLinkedListNode<T>>) {
        let node = node.into();
        let pos = self
            .blocks
            .iter()
            .position(|x| x.start > node.start)
            .map(|x| x - 1)
            .unwrap_or(self.blocks.len());
        self.blocks.insert(pos, node);
    }

    pub fn try_get(&self) -> Option<&T> {
        if self.blocks.len() <= 1 {
            return Some(&self.data);
        }
        let now = Local::now().time();
        if now > self.next_tick {
            None
        } else {
            Some(&self.data)
        }
    }

    pub fn try_get_mut(&mut self) -> Option<&mut T> {
        if self.blocks.len() <= 1 {
            return Some(&mut self.data);
        }
        let now = Local::now().time();
        if now > self.next_tick {
            None
        } else {
            Some(&mut self.data)
        }
    }

    pub fn forward(&mut self) -> bool {
        let tail = self.blocks.back().unwrap();
        (tail.leave)(&mut self.data);
        let tail_start = tail.start;
        let front = self.blocks.pop_front().unwrap();
        let is_going_to_next_day = front.start < tail_start;
        (front.enter)(&mut self.data);
        self.next_tick = front.start;
        self.blocks.push_back(front);
        is_going_to_next_day
    }

    pub fn forward_to(&mut self, time: NaiveTime) {
        let is_going_to_next_day
        while !self.is_outdated_by(time) {
            self.forward();
        }
    }

    pub fn forward_to_now(&mut self) {
        self.forward_to(Local::now().time());
    }

    #[inline]
    pub fn is_outdated_by(&self, time: NaiveTime) -> bool {
        time >= self.next_tick
    }

    #[inline]
    pub fn is_outdated_by_now(&self) -> bool {
        self.is_outdated_by(Local::now().time())
    }

    #[inline]
    pub fn data(&self) -> &T {
        &self.data
    }

    #[inline]
    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn get(&mut self) -> &T {
        self.forward_to_now();
        self.data()
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.forward_to_now();
        self.data_mut()
    }
}
