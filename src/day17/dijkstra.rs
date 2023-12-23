use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Eq, PartialEq, Debug, Default, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct Coordinates(pub usize, pub usize);

impl Coordinates {
    pub fn new(line: usize, column: usize) -> Self {
        Coordinates(line, column)
    }
}

/// This function finds the cheapest path between two points in a matrix
/// Each point is represented by a (line, column) tuple, and the values in matrix[line][column] is the cost of the path.
/// The function returns a vector of coordinates representing the path.
pub fn dijkstra_path(
    matrix: &Vec<Vec<u32>>,
    start: Coordinates,
    end: Coordinates,
) -> Vec<Coordinates> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    // Priority queue to store vertices with their respective costs
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(State {
        cost: 0,
        position: start,
    });

    // HashMap to track the cost to reach each vertex
    let mut costs: HashMap<Coordinates, u32> = HashMap::new();
    costs.insert(start, 0);

    // HashSet to track visited vertices
    let mut visited: HashSet<Coordinates> = HashSet::new();

    // HashMap to store the previous vertex in the cheapest path
    let mut previous: HashMap<Coordinates, Coordinates> = HashMap::new();

    // Dijkstra's algorithm
    while let Some(State { cost, position }) = priority_queue.pop() {
        if position == end {
            // Reconstruct the path if we reached the destination
            let mut path = Vec::new();
            let mut current = end;

            while let Some(&prev) = previous.get(&current) {
                path.push(current);
                current = prev;
            }

            path.push(start);
            path.reverse();
            return path;
        }

        if visited.contains(&position) {
            continue;
        }

        visited.insert(position);

        // Explore neighbors
        for neighbor in neighbors(position, rows, cols) {
            let new_cost = cost + matrix[neighbor.0][neighbor.1];

            if !costs.contains_key(&neighbor) || new_cost < *costs.get(&neighbor).unwrap() {
                costs.insert(neighbor, new_cost);
                priority_queue.push(State {
                    cost: new_cost,
                    position: neighbor,
                });
                previous.insert(neighbor, position);
            }
        }
    }

    // If no path is found, return an empty vector
    Vec::new()
}

// Helper struct to represent the state of a vertex in the priority queue
#[derive(Debug, Eq, PartialEq)]
struct State {
    cost: u32,
    position: Coordinates,
}

// Implement Ord for State to enable comparison in the priority queue
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

// Implement PartialOrd for State to enable comparison in the priority queue
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Helper function to get valid neighbors of a position
fn neighbors(position: Coordinates, rows: usize, cols: usize) -> Vec<Coordinates> {
    let mut result = Vec::new();

    if position.0 > 0 {
        result.push(Coordinates(position.0 - 1, position.1));
    }

    if position.0 < rows - 1 {
        result.push(Coordinates(position.0 + 1, position.1));
    }

    if position.1 > 0 {
        result.push(Coordinates(position.0, position.1 - 1));
    }

    if position.1 < cols - 1 {
        result.push(Coordinates(position.0, position.1 + 1));
    }

    result
}
