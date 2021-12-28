use std::{
    collections::{BinaryHeap, HashMap},
    marker::PhantomData,
};

use crate::{
    astar::GridStrategy,
    grid::{GridRef, Position},
};

#[derive(Debug)]
pub struct Search<P> {
    strategy: PhantomData<P>,
    end: Position,
    queue: BinaryHeap<Node>,
    costs: HashMap<Position, usize>,
}

impl<P> Search<P> {
    pub fn new() -> Self {
        Self {
            strategy: PhantomData,
            end: Position { x: 0, y: 0 },
            queue: BinaryHeap::new(),
            costs: HashMap::new(),
        }
    }

    pub fn next(&mut self) -> Option<Position> {
        self.queue.pop().map(|n| n.position)
    }
}

impl<P: GridStrategy> Search<P> {
    pub fn start(grid: GridRef) -> Self {
        let mut search = Self::new();

        search.end = P::end(grid);
        search.costs.insert(Node::start().position, 0);
        search.queue.push(Node::start());

        search
    }

    pub fn cost(&self, position: &Position) -> Option<usize> {
        self.costs.get(position).copied()
    }

    pub fn visit(&mut self, position: &Position, cost: usize) {
        if self.should_update_position(position, cost) {
            self.costs.insert(position.clone(), cost);
            self.queue.push(Node {
                position: position.clone(),
                priority: cost + self.distance_from_end(position),
            })
        }
    }

    pub fn is_done(&self, position: &Position) -> bool {
        position == &self.end
    }

    pub fn distance_from_end(&self, position: &Position) -> usize {
        position.distance(&self.end)
    }

    fn should_update_position(&self, position: &Position, cost: usize) -> bool {
        self.costs.get(position).map(|c| cost < *c).unwrap_or(true)
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Node {
    position: Position,
    priority: usize,
}

impl Node {
    pub const fn start() -> Self {
        Node {
            position: Position { x: 0, y: 0 },
            priority: 0,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
