use noisy_float::prelude::*;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, Read};

fn main() {
  println!("Hello, world!");
  let berlin = "
NAME: berlin52
TYPE: TSP
COMMENT: 52 locations in Berlin (Groetschel)
DIMENSION: 52
EDGE_WEIGHT_TYPE: EUC_2D
NODE_COORD_SECTION
1 565.0 575.0
2 25.0 185.0
3 345.0 750.0
4 945.0 685.0
5 845.0 655.0
6 880.0 660.0
7 25.0 230.0
8 525.0 1000.0
9 580.0 1175.0
10 650.0 1130.0
";
  // 11 1605.0 620.0
  // 12 1220.0 580.0
  // 13 1465.0 200.0
  // 14 1530.0 5.0
  // 15 845.0 680.0
  // 16 725.0 370.0
  // 17 145.0 665.0
  // 18 415.0 635.0
  // 19 510.0 875.0
  // 20 560.0 365.0
  // 21 300.0 465.0
  // 22 520.0 585.0
  // 23 480.0 415.0
  // 24 835.0 625.0
  // 25 975.0 580.0
  // 26 1215.0 245.0
  // 27 1320.0 315.0
  // 28 1250.0 400.0
  // 29 660.0 180.0
  // 30 410.0 250.0
  // 31 420.0 555.0
  // 32 575.0 665.0
  // 33 1150.0 1160.0
  // 34 700.0 580.0
  // 35 685.0 595.0
  // 36 685.0 610.0
  // 37 770.0 610.0
  // 38 795.0 645.0
  // 39 720.0 635.0
  // 40 760.0 650.0
  // 41 475.0 960.0
  // 42 95.0 260.0
  // 43 875.0 920.0
  // 44 700.0 500.0
  // 45 555.0 815.0
  // 46 830.0 485.0
  // 47 1170.0 65.0
  // 48 830.0 610.0
  // 49 605.0 625.0
  // 50 595.0 360.0
  // 51 1340.0 725.0
  // 52 1740.0 245.0
  // let mut input = String::new();
  // io::stdin().read_to_string(&mut input);
  let mut tsp = TSP::new(berlin.to_string());
  tsp.solve();
}
// simple exact TSP solver based on branch-and-bound/Held--Karp
#[derive(Debug, Clone)]
struct TSP {
  n: usize,
  x: Vec<N32>,
  y: Vec<N32>,
  cost: Vec<Vec<N32>>,
  cost_with_pi: Vec<Vec<N32>>,
  best: Node,
}
#[derive(Eq, Default, Debug, Clone)]
struct Node {
  excluded: Vec<Vec<bool>>,
  pi: Vec<N32>,
  lower_bound: N32,
  degree: Vec<usize>,
  parent: Vec<usize>,
}
impl Ord for Node {
  fn cmp(&self, other: &Node) -> Ordering {
    self.lower_bound.cmp(&other.lower_bound)
  }
}
impl PartialEq for Node {
  fn eq(&self, other: &Self) -> bool {
    self.lower_bound == other.lower_bound
  }
}
impl PartialOrd for Node {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
impl TSP {
  fn addEdge(&self, node: &mut Node, i: usize, j: usize) {
    node.lower_bound += self.cost_with_pi[i][j];
    node.degree[i] += 1;
    node.degree[j] += 1;
  }
  fn exclude(&mut self, node: &mut Node, i: usize, j: usize) -> Node {
    let mut child: Node = Default::default();
    child.pi = vec![n32(0.0); self.n];
    child.parent = vec![0; self.n];
    child.excluded = node.excluded.clone();
    child.excluded[i] = node.excluded[i].clone();
    child.excluded[j] = node.excluded[j].clone();
    child.excluded[i][j] = true;
    child.excluded[j][i] = true;
    self.computeHeldKarp(&mut child);
    child
  }
  fn computeHeldKarp(&mut self, node: &mut Node) {
    node.lower_bound = n32(std::f32::MIN);
    node.degree = vec![0; self.n];
    node.parent = vec![0; self.n];
    let mut lambda = n32(0.1);
    while lambda > 1e-06 {
      let previous_lower = node.lower_bound;
      self.computeOneTree(node);
      if node.lower_bound >= self.best.lower_bound {
        return;
      }
      if node.lower_bound >= previous_lower {
        lambda *= 0.9;
      }
      let mut denom = 0;
      for i in 1..self.n {
        let deg2 = (node.degree[i as usize] as i32) - 2;
        denom += deg2 * deg2;
      }
      // println!("---DENOM : {}", denom);
      if denom == 0 {
        return;
      }
      // println!("---lambda, lower_bound : {}, {}", lambda, node.lower_bound);
      let t: N32 = lambda * node.lower_bound / n32(denom as f32);
      for i in 1..self.n {
        node.pi[i as usize] += t * n32(((node.degree[i as usize] as i32) - 2) as f32);
      }
    }
  }
  fn computeOneTree(&mut self, node: &mut Node) {
    node.lower_bound = n32(0.0);
    node.degree = vec![0; self.n];
    for i in 0..self.n {
      for j in 0..self.n {
        self.cost_with_pi[i][j] = if node.excluded[i][j] {
          n32(std::f32::MAX)
        } else {
          self.cost[i][j] + node.pi[i] + node.pi[j]
        }
      }
    }
    // find the two cheapest edges from 0
    let (mut first_neighbor, mut second_neighbor) =
      if self.cost_with_pi[0][2] < self.cost_with_pi[0][1] {
        (2, 1)
      } else {
        (1, 2)
      };
    //find the top two smallest edges from 0, keeping track of the cheapest and 2nd cheapest.
    for j in 3..self.n {
      let j_cost = self.cost_with_pi[0][j];
      if j_cost < self.cost_with_pi[0][second_neighbor] {
        if j_cost < self.cost_with_pi[0][first_neighbor] {
          second_neighbor = first_neighbor;
          first_neighbor = j;
        } else {
          second_neighbor = j;
        }
      }
    }
    self.addEdge(node, 0, first_neighbor);
    node.parent = vec![first_neighbor; self.n];
    node.parent[first_neighbor] = 0;
    // compute the minimum spanning tree on nodes 1..n-1
    let mut min_cost = self.cost_with_pi[first_neighbor].clone();
    for _k in 2..self.n {
      let mut i = node.degree.iter().position(|&degree| degree == 0).unwrap();
      for j in (i + 1)..self.n {
        if node.degree[j] == 0 && min_cost[j] < min_cost[i] {
          i = j;
        }
      }
      self.addEdge(node, node.parent[i], i);
      for j in 1..self.n {
        if node.degree[j] == 0 && self.cost_with_pi[i][j] < min_cost[j] {
          min_cost[j] = self.cost_with_pi[i][j];
          node.parent[j] = i;
        }
      }
    }
    self.addEdge(node, 0, second_neighbor);
    node.parent[0] = second_neighbor;
    // println!(
    //   "Setting lower bound from {} to {}",
    //   node.lower_bound,
    //   node.lower_bound.round(),
    // );
    node.lower_bound = node.lower_bound.round();
  }
  fn solve(&mut self) {
    self.best = self.new_node();
    self.best.lower_bound = n32(std::f32::MAX);
    let mut currentNode: Node = self.new_node();
    println!("{:?}", self);
    self.computeHeldKarp(&mut currentNode);
    let mut pq = BinaryHeap::new();
    loop {
      loop {
        let mut iopt: Option<usize> = None;
        for j in 0..self.n {
          if currentNode.degree[j] > 2
            && (iopt.is_none() || currentNode.degree[j] < currentNode.degree[iopt.unwrap()])
          {
            iopt = Some(j);
          }
        }
        match iopt {
          None => {
            if currentNode.lower_bound < self.best.lower_bound {
              self.best = currentNode.clone();
              // println!("{}", self.best_lb)
            }
            break;
          }
          Some(i) => {
            println!(".");
            let mut children: BinaryHeap<Node> = BinaryHeap::new();
            let parent_i = currentNode.parent[i];
            children.push(self.exclude(&mut currentNode, i, parent_i));
            for j in 0..self.n {
              if currentNode.parent[j] == i {
                children.push(self.exclude(&mut currentNode, i, j));
              }
            }
            currentNode = children.pop().unwrap();
            pq.append(&mut children);
            if currentNode.lower_bound >= self.best.lower_bound {
              break;
            }
          }
        }
      }
      match pq.pop() {
        None => {
          println!("Breaking because pq.pop returned None");
          break;
        }
        Some(new_node) => currentNode = new_node,
      };
      // println!("There are {} nodes to visit", pq.len());
      if currentNode.lower_bound > self.best.lower_bound {
        println!(
          "Breaking because current node lower bound {} is greater than best lower bound of {}",
          currentNode.lower_bound, self.best.lower_bound
        );
        break;
      }
    }
    println!("best lower bound is {}", self.best.lower_bound);
    let mut j = 0;
    let mut i = self.best.parent[0];
    let mut cost = n32(0.0);
    while i != 0 {
      cost += self.cost_with_pi[i][j];
      i = self.best.parent[j];
      println!(
        "{}->{}\t{}\t{}\t{}\t{}",
        j,
        i,
        self.x[j],
        self.y[j],
        self.x[i] - self.x[j],
        self.y[i] - self.y[j]
      );
      j = i;
    }
    println!("The trip cost is {}", cost)
  }
  fn new(input: String) -> TSP {
    // let dimension_ex = Regex::new(r"DIMENSION: *([0-9]+)").unwrap();
    println!("Getting the dimension");
    let n: usize = 26; //dimension_ex.find(&input).unwrap().as_str().parse().unwrap();
    println!("Got n = {}", n);
    let mut cost = vec![vec![n32(0.0); n]; n];
    let mut x = vec![n32(0.0); n];
    let x: Vec<N32> = vec![
      n32(565.0),
      n32(25.0),
      n32(345.0),
      n32(945.0),
      n32(845.0),
      n32(880.0),
      n32(25.0),
      n32(525.0),
      n32(580.0),
      n32(650.0),
      n32(1605.0),
      n32(1220.0),
      n32(1465.0),
      n32(1530.0),
      n32(845.0),
      n32(725.0),
      n32(145.0),
      n32(415.0),
      n32(510.0),
      n32(560.0),
      n32(300.0),
      n32(520.0),
      n32(480.0),
      n32(835.0),
      n32(975.0),
      n32(1215.0),
      n32(1320.0),
      n32(1250.0),
      n32(660.0),
      n32(410.0),
      n32(420.0),
      n32(575.0),
      n32(1150.0),
      n32(700.0),
      n32(685.0),
      n32(685.0),
      n32(770.0),
      n32(795.0),
      n32(720.0),
      n32(760.0),
      n32(475.0),
      n32(95.0),
      n32(875.0),
      n32(700.0),
      n32(555.0),
      n32(830.0),
      n32(1170.0),
      n32(830.0),
      n32(605.0),
      n32(595.0),
      n32(1340.0),
      n32(1740.0),
    ];
    let y: Vec<N32> = vec![
      n32(575.0),
      n32(185.0),
      n32(750.0),
      n32(685.0),
      n32(655.0),
      n32(660.0),
      n32(230.0),
      n32(1000.0),
      n32(1175.0),
      n32(1130.0),
      n32(620.0),
      n32(580.0),
      n32(200.0),
      n32(5.0),
      n32(680.0),
      n32(370.0),
      n32(665.0),
      n32(635.0),
      n32(875.0),
      n32(365.0),
      n32(465.0),
      n32(585.0),
      n32(415.0),
      n32(625.0),
      n32(580.0),
      n32(245.0),
      n32(315.0),
      n32(400.0),
      n32(180.0),
      n32(250.0),
      n32(555.0),
      n32(665.0),
      n32(1160.0),
      n32(580.0),
      n32(595.0),
      n32(610.0),
      n32(610.0),
      n32(645.0),
      n32(635.0),
      n32(650.0),
      n32(960.0),
      n32(260.0),
      n32(920.0),
      n32(500.0),
      n32(815.0),
      n32(485.0),
      n32(65.0),
      n32(610.0),
      n32(625.0),
      n32(360.0),
      n32(725.0),
      n32(245.0),
    ];
    // TSPLIB distances are rounded to the nearest integer to avoid the sum of square roots problem
    for i in 0..n {
      for j in 0..n {
        let dx = x[i] - x[j];
        let dy = y[i] - y[j];
        cost[i][j] = (dx * dx + dy * dy).sqrt().trunc();
      }
    }
    TSP {
      n,
      x,
      y,
      cost,
      cost_with_pi: vec![vec![n32(0.0); n]; n],
      best: Default::default(),
    }
  }
  fn new_node(&self) -> Node {
    Node {
      excluded: vec![vec![false; self.n]; self.n],
      pi: vec![n32(0.0); self.n],
      lower_bound: n32(std::f32::MAX),
      degree: vec![0; self.n],
      parent: vec![0; self.n],
    }
  }
}
