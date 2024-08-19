use daggy::petgraph::stable_graph::{edge_index, node_index};
use daggy::Walker;
use pendulum::PendulumAgent as CurrentAgent;
use rand::distributions::{Uniform, WeightedError, WeightedIndex};
use rand::prelude::*;
use std::marker::PhantomData;
use std::sync::mpsc::Sender;

pub mod pendulum;

#[derive(Clone, Debug)]
struct Node {
    value: f32,
    bias: f32,
}

impl Node {
    fn random() -> Self {
        Self {
            value: 0.0,
            bias: thread_rng().gen_range(-1.0..=1.0),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Edge {
    weight: f32,
}

impl Edge {
    fn new(weight: f32) -> Self {
        Self { weight }
    }

    fn random() -> Self {
        Self::new(thread_rng().gen_range(-1.0..=1.0))
    }
}

#[derive(Clone)]
pub struct Agent<I: Inputs, O: Outputs> {
    dag: daggy::Dag<Node, Edge>,
    _inputs: PhantomData<I>,
    _outputs: PhantomData<O>,
}

impl<I: Inputs, O: Outputs> Agent<I, O> {
    fn new() -> Self {
        let mut dag = daggy::Dag::new();
        for _ in 0..I::COUNT {
            dag.add_node(Node::random());
        }
        for _ in 0..O::COUNT {
            dag.add_node(Node::random());
        }
        Self {
            dag,
            _inputs: PhantomData,
            _outputs: PhantomData,
        }
    }

    fn mutate(&mut self) {
        let mut rng = thread_rng();
        for edge in self.dag.edge_weights_mut() {
            if rng.gen_bool(0.2) {
                if rng.gen_bool(0.2) {
                    *edge = Edge::random();
                } else if rng.gen_bool(0.25) {
                    edge.weight += rng.gen_range(-1.0..=1.0);
                } else {
                    edge.weight += 0.01 * rng.gen_range(-1.0..=1.0);
                }
            }
        }
        for node in self.dag.node_weights_mut() {
            if rng.gen_bool(0.2) {
                if rng.gen_bool(0.2) {
                    *node = Node::random();
                } else if rng.gen_bool(0.25) {
                    node.bias += rng.gen_range(-1.0..=1.0);
                } else {
                    node.bias += 0.01 * rng.gen_range(-1.0..=1.0);
                }
            }
        }
        if self.dag.node_count() < 30 && rng.gen_bool(0.25) {
            self.new_node();
        }
        if rng.gen_bool(0.25) {
            self.new_connection();
        }
    }

    fn new_connection(&mut self) {
        let mut rng = thread_rng();
        let count = self.dag.node_count();
        let mut i = rng.gen_range(0..count - O::COUNT);
        if i >= I::COUNT {
            i += O::COUNT;
        }
        let source_node = node_index(i);
        let i = rng.gen_range(I::COUNT..count);
        let target_node = node_index(i);
        if self.dag.find_edge(source_node, target_node).is_none() {
            self.dag
                .add_edge(source_node, target_node, Edge::random())
                .ok();
        }
    }

    fn new_node(&mut self) {
        let count = self.dag.edge_count();
        if count == 0 {
            return;
        }
        let i = thread_rng().gen_range(0..count);
        let (edge, parent, target) = {
            let edge = self.dag.raw_edges().get(i).unwrap();
            (edge.weight, edge.source(), edge.target())
        };
        self.dag.remove_edge(edge_index(i)).unwrap();
        let (_, node_index) = self.dag.add_child(parent, edge, Node::random());
        self.dag
            .add_edge(node_index, target, Edge::new(1.0))
            .unwrap();
    }

    pub fn choose(&mut self, inputs: I) -> O {
        for node in self.dag.node_weights_mut() {
            node.value = 0.0;
        }
        let sorted_nodes = daggy::petgraph::algo::toposort(self.dag.graph(), None).unwrap();

        for source_node in sorted_nodes {
            let source_neuron_value = {
                let source_neuron = self.dag.node_weight_mut(source_node).unwrap();
                source_neuron.bias
                    + (if source_node.index() < I::COUNT {
                        inputs.get(source_node.index())
                    } else {
                        source_neuron.value.tanh()
                    })
            };

            let mut children = self.dag.children(source_node);
            while let Some((edge, target_node)) = children.walk_next(&self.dag) {
                let weight = self.dag.edge_weight(edge).unwrap().weight;
                let target_neuron = self.dag.node_weight_mut(target_node).unwrap();
                target_neuron.value += source_neuron_value * weight;
            }
        }

        let count = self.dag.node_count();
        O::from_iter(
            (count - O::COUNT..count).map(|i| self.dag.node_weight(node_index(i)).unwrap().value),
        )
    }
}

pub trait Inputs {
    const COUNT: usize;
    fn get(&self, index: usize) -> f32;
}

pub trait Outputs {
    const COUNT: usize;
    fn from_iter<I: Iterator<Item = f32>>(it: I) -> Self;
}

pub struct Ml {
    sender: Sender<CurrentAgent>,
    best_score: f32,
}

impl Ml {
    pub fn new(sender: Sender<CurrentAgent>) -> Self {
        Self {
            sender,
            best_score: 0.0,
        }
    }

    pub fn run_experiment(&mut self) {
        let mut agents: Vec<CurrentAgent> = (0..10).map(|_| CurrentAgent::new()).collect();
        loop {
            agents = self.selection(agents);
        }
    }

    fn selection(&mut self, mut agents: Vec<CurrentAgent>) -> Vec<CurrentAgent> {
        let mut rng = thread_rng();

        use rayon::prelude::*;
        let mut scores_and_agents: Vec<(f32, CurrentAgent)> = agents
            .into_par_iter()
            .map(|mut agent| (pendulum::run_simulation(&mut agent), agent))
            .collect();
        scores_and_agents.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let (best_score, best_agent) = scores_and_agents.last().unwrap();
        if *best_score > self.best_score {
            self.best_score = *best_score;
            self.sender.send(best_agent.clone()).unwrap();
            println!("New best score: {}", best_score);
        }

        agents = Vec::with_capacity(scores_and_agents.len());
        agents.push(scores_and_agents.pop().unwrap().1);
        agents.push(scores_and_agents.pop().unwrap().1);
        agents.push(scores_and_agents.pop().unwrap().1);

        let scores: Vec<f32> = scores_and_agents.iter().map(|x| x.0).collect();
        let dist = WeightedIndex::new(&scores);
        if let Err(WeightedError::AllWeightsZero) = dist {
            let dist = Uniform::new(0, scores_and_agents.len());
            for _ in 0..scores_and_agents.len() {
                let mut agent = scores_and_agents[dist.sample(&mut rng)].1.clone();
                agent.mutate();
                agents.push(agent);
            }
        } else {
            let dist = dist.unwrap();
            for _ in 0..scores_and_agents.len() {
                let mut agent = scores_and_agents[dist.sample(&mut rng)].1.clone();
                agent.mutate();
                agents.push(agent);
            }
        }
        agents
    }
}
