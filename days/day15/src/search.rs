use std::collections::{BinaryHeap, HashMap};

use crate::grid::Position;

#[derive(Debug)]
pub struct Search {
    end: Position,
    queue: BinaryHeap<Node>,
    costs: HashMap<Position, usize>,
}

impl Search {
    pub fn start(end: Position) -> Self {
        let mut search = Self {
            end,
            queue: BinaryHeap::new(),
            costs: HashMap::new(),
        };

        search.costs.insert(Node::start().position, 0);
        search.queue.push(Node::start());

        search
    }

    pub fn cost(&self, position: &Position) -> usize {
        self.costs
            .get(position)
            .copied()
            .expect("positions passed into cost should have come from this search's next method, which means that they already have a cost")
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

    fn distance_from_end(&self, position: &Position) -> usize {
        position.distance(&self.end)
    }

    fn should_update_position(&self, position: &Position, cost: usize) -> bool {
        self.costs.get(position).map(|c| cost < *c).unwrap_or(true)
    }
}

impl Iterator for Search {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop().map(|n| n.position)
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
            position: Position::ZERO,
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
