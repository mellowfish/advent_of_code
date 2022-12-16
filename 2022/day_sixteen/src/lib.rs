use std::collections::HashMap;

#[derive(Clone)]
struct Valve {
    index: usize,
    name: String,
    neighbors: Vec<String>,
    flow_rate: usize,
    is_open: bool,
    total_flow: usize
}

impl Valve {
    fn new(index: usize, input: &str) -> Self {
        let mut input = input.strip_prefix("Valve ").unwrap();
        let (name, flow_rate_and_neighbors) = input.split_once(" has flow rate=").unwrap();
        let (flow_rate_str, mut neighbors_str) = flow_rate_and_neighbors.split_once("; tunnel").unwrap();
        let flow_rate = flow_rate_str.parse().unwrap();
        if neighbors_str[0..1].eq("s") {
            neighbors_str = &neighbors_str[17..neighbors_str.len()];
        } else {
            neighbors_str = &neighbors_str[16..neighbors_str.len()];
        }
        let neighbors = neighbors_str.split(", ").map(str::to_string).collect();

        Self { is_open: false, total_flow: 0, name: name.to_string(), flow_rate, neighbors, index }
    }

    fn can_flow(&self) -> bool {
        self.total_flow > 0
    }

    fn open(&mut self) {
        self.is_open = true
    }
}

struct PipeSystem {
    valves: Vec<Valve>,
    named_valve_indices: HashMap<String, usize>,
    flowable_valve_indices: Vec<usize>,
    distances: Vec<Vec<usize>>,
    previous_valves: Vec<Vec<usize>>
}

impl PipeSystem {
    fn new(input: &str) -> Self {
        let mut lines: Vec<&str> = input.lines().collect();
        lines.sort();
        let num_valves = lines.len();

        let mut pipe_system = Self {
            valves: lines.into_iter().enumerate().map(|(index, line)| Valve::new(index, line)).collect(),
            named_valve_indices: HashMap::new(),
            flowable_valve_indices: vec![],
            distances: vec![vec![usize::MAX; num_valves]; num_valves],
            previous_valves: vec![vec![usize::MAX; num_valves]; num_valves]
        };
        pipe_system.finish_initialization();
        pipe_system.calculate_shortest_paths();

        pipe_system
    }

    fn finish_initialization(&mut self) {
        for (index, valve) in self.valves.iter().enumerate() {
            if valve.can_flow() {
                self.flowable_valve_indices.push(index);
            }
            self.named_valve_indices.insert(valve.name.clone(), index);
        }
    }

    fn calculate_shortest_paths(&mut self) {
        for start_index in 0..self.valves.len() {
            self.distances[start_index][start_index] = 0; // to get min traversal to work correctly...

            let mut unvisited_valves : Vec<usize> = (0..self.valves.len()).collect();
            while !unvisited_valves.is_empty() {
                unvisited_valves.sort_by_key(|index| usize::MAX - self.distances[start_index][*index]); // reverse sort by distance
                if unvisited_valves.iter().all(|index| self.distances[start_index][*index] == usize::MAX) {
                    break; // untraversable nodes
                }

                let current_index = unvisited_valves.pop().unwrap();
                if self.distances[start_index][current_index] == usize::MAX {
                    dbg!(start_index, current_index);
                    panic!("Unexpected visit to infinite node")
                }

                let current_valve = &self.valves[current_index];

                for neighbor in current_valve.neighbors.iter() {
                    let neighbor_index = self.valve_named(neighbor).index;

                    if unvisited_valves.contains(&neighbor_index) {
                        let new_distance = self.distances[start_index][current_index] + 1;
                        if new_distance < self.distances[start_index][neighbor_index] {
                            self.distances[start_index][neighbor_index] = new_distance;
                            self.previous_valves[start_index][neighbor_index] = current_index;
                        }
                    }
                }
            }

            self.distances[start_index][start_index] = 1; // idle
        }
    }

    fn index_for_name(&self, name: &str) -> usize {
        *self.named_valve_indices.get(name).unwrap()
    }

    fn valve_named(&self, name: &str) -> &Valve {
        &self.valves[self.index_for_name(name)]
    }

    fn distance_from(&self, from: usize, to: usize) -> usize {
        self.distances[from][to]
    }

    fn distance_from_named(&self, from: &str, to: &str) -> usize {
        self.distance_from(self.index_for_name(from), self.index_for_name(to))
    }

    #[allow(dead_code)]
    fn print_distances(&self) {
        // HEADER
        print!("   ");
        for (index, valve) in self.valves.iter().enumerate() {
            print!("{} ", valve.name)
        }
        println!();

        for (from_index, from_valve) in self.valves.iter().enumerate() {
            print!("{} ", from_valve.name);
            for (to_index, _to_valve) in self.valves.iter().enumerate() {
                print!("{:2} ", self.distances[from_index][to_index]);
            }
            println!();
        }
        println!();
    }
}

struct Simulation {
    pipe_system: PipeSystem,
    tick: usize,
    current_valve_index: usize,
    target_valve_index: usize,
    total_pressure_released: usize
}

impl Simulation {
    fn new(pipe_system: PipeSystem) -> Self {
        Self {
            pipe_system,
            tick: 0,
            current_valve_index: 0,
            target_valve_index: 0,
            total_pressure_released: 0
        }
    }

    fn ticks_remaining(&self) -> usize {
        30 - self.tick
    }

    fn end_tick(&mut self) {
        self.tick += 1
    }

    fn next_target(&self) -> Option<usize> {
        self.pipe_system.flowable_valve_indices.iter().filter_map(|&index| {
            let valve = &self.pipe_system.valves[index];
            if valve.is_open {
                return None;
            }
            let time_open = self.ticks_remaining() - (self.pipe_system.distance_from(self.current_valve_index, index) + 1);
            if time_open < 1 {
                return None;
            }

            Some((time_open * valve.flow_rate, valve))
        }).min_by_key(|(score, _valve)| *score).map(|(_score, valve)| valve.index)
    }

    fn release_max_pressure(&mut self) -> usize {
        while self.ticks_remaining() > 0 {
            match self.next_target() {
                Some(index) => {
                    self.travel_to(index);
                    self.open_valve();
                },
                None => {
                    self.meditate_on_ones_mortality();
                    break;
                }
            }
        }

        self.total_pressure_released
    }

    fn pressure_released(&self) -> usize {
        self.pipe_system.valves.iter().filter_map(|valve| {
            if valve.is_open {
                Some(valve.flow_rate)
            } else {
                None
            }
        }).sum()
    }

    fn record_pressure_released(&mut self) {
        self.total_pressure_released += self.pressure_released();
    }

    fn meditate_on_ones_mortality(&mut self) {
        while self.ticks_remaining() > 0 {
            self.record_pressure_released();
            self.end_tick()
        }
    }

    fn travel_to(&mut self, index: usize) {
        while self.current_valve_index != index {
            todo!();
        }
    }

    fn open_valve(&mut self) {
        self.pipe_system.valves[self.current_valve_index].open()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_one_example() {
        let pipe_system = PipeSystem::new(fs::read_to_string("example_input.txt").unwrap().as_str());
        let mut simulation = Simulation::new(pipe_system);
        assert_eq!(simulation.release_max_pressure(), 1651);
    }
}
