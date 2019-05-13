use noisy_float::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, Read};

fn main() {
  println!("Hello, world!");
  // read the input in TSPLIB format
  // assume TYPE: TSP, EDGE_WEIGHT_TYPE: EUC_2D
  // no error checking
  let mut input = String::new();
  io::stdin().read_to_string(&mut input)?;
  let tsp = TSP::new(input);
  tsp.solve();
}
// simple exact TSP solver based on branch-and-bound/Held--Karp
struct TSP {
  n: usize,
  x: Vec<N32>,
  y: Vec<N32>,
  cost: Vec<Vec<N32>>,
  cost_with_pi: Vec<Vec<N32>>,
  best: Node,
}
#[derive(Eq, Default)]
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
  fn exclude(&self, node: &mut Node, i: usize, j: usize) -> Node {
    let mut child: Node = Default::default();
    child.excluded = node.excluded.clone();
    child.excluded[i] = node.excluded[i].clone();
    child.excluded[j] = node.excluded[j].clone();
    child.excluded[i][j] = true;
    child.excluded[j][i] = true;
    self.computeHeldKarp(&mut child);
    child
  }
  fn computeHeldKarp(&self, node: &mut Node) {
    node.lower_bound = n32(std::f32::MIN);
    node.degree = vec![0; self.n];
    node.parent = vec![0; self.n];
    let mut lambda = n32(0.1);
    while lambda > 1e-06 {
      let previous_lower = node.lower_bound;
      //      computeOneTree(node);
      if !(node.lower_bound < self.best.lower_bound) {
        return;
      }
      if !(node.lower_bound < previous_lower) {
        lambda *= 0.9;
      }
      let mut denom = 0;
      for i in 1..self.n {
        let degree = node.degree[i as usize];
        denom += (degree - 2) ^ 2;
      }
      if denom == 0 {
        return;
      }
      let t: N32 = lambda * node.lower_bound / n32(denom as f32);
      for i in 1..self.n {
        node.pi[i as usize] += t * n32((node.degree[i as usize] - 2) as f32);
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
    node.parent = vec![self.n, first_neighbor];
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
    node.lower_bound = node.lower_bound.round();
  }
  fn solve(&mut self) {
    self.best.lower_bound = n32(std::f32::MAX);
    let mut currentNode: Node = Default::default();
    currentNode.excluded = vec![vec![false; self.n]; self.n];
    //  self.cost_with_pi = new double[n][n];
    self.computeHeldKarp(&mut currentNode);
    let mut pq = BinaryHeap::new();

    //  PriorityQueue<Node> pq = new PriorityQueue<Node>(11, new NodeComparator());
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
        if iopt.is_none() {
          if currentNode.lower_bound < self.best.lower_bound {
            self.best = currentNode;
            // System.err.printf("%.0f", bestNode.lowerBound);
          }
          break;
        }
        let i = iopt.unwrap();
        //        System.err.printf(".");
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
      //      System.err.printf("%n");
      let new_current = pq.pop(); //dont unwrap, break on empty;
      if new_current.is_none() {
        break;
      }
      currentNode = new_current.unwrap();
      if currentNode.lower_bound >= self.best.lower_bound {
        break;
      }
    }
    // output suitable for gnuplot
    //    System.out.printf("# %.0f%n", bestNode.lowerBound);
    let mut j = 0;
    loop {
      let i = self.best.parent[j];
      print!(
        "{}\t{}\t{}\t{}%n",
        self.x[j],
        self.y[j],
        self.x[i] - self.x[j],
        self.y[i] - self.y[j]
      );
      j = i;
      if j == 0 {
        break;
      }
    }
  }
fn new(input: String) -> Self {
  let mut new = TS
//    //Pattern specification = Pattern.compile("\\s*([A-Z_]+)\\s*(:\\s*([0-9]+))?\\s*");
//    //Pattern data = Pattern.compile("\\s*([0-9]+)\\s+([-+.0-9Ee]+)\\s+([-+.0-9Ee]+)\\s*");
   for line in input.lines() {
//    while ((line = in.readLine()) != null) {
//      Matcher m = specification.matcher(line);
//      if (!m.matches()) continue;
//      String keyword = m.group(1);
//      if (keyword.equals("DIMENSION")) {
//        n = Integer.parseInt(m.group(3));
//        cost = new double[n][n];
//      } else if (keyword.equals("NODE_COORD_SECTION")) {
//        x = new double[n];
//        y = new double[n];
//        for (int k = 0; k < n; k++) {
//          line = in.readLine();
//          m = data.matcher(line);
//          m.matches();
//          int i = Integer.parseInt(m.group(1)) - 1;
//          x[i] = Double.parseDouble(m.group(2));
//          y[i] = Double.parseDouble(m.group(3));
//        }
//        // TSPLIB distances are rounded to the nearest integer to avoid the sum of square roots problem
//        for (int i = 0; i < n; i++) {
//          for (int j = 0; j < n; j++) {
//            double dx = x[i] - x[j];
//            double dy = y[i] - y[j];
//            cost[i][j] = Math.rint(Math.sqrt(dx * dx + dy * dy));
//          }
//        }
//      }
//    }
 }

}