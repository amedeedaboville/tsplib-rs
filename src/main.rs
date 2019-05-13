use noisy_float::prelude::*;
use std::cmp::Ordering;

fn main() {
  println!("Hello, world!");
  // read the input in TSPLIB format
  // assume TYPE: TSP, EDGE_WEIGHT_TYPE: EUC_2D
  // no error checking
  //TSP tsp = new TSP();
  //tsp.readInput(new InputStreamReader(System.in));
  //tsp.solve();
}
// simple exact TSP solver based on branch-and-bound/Held--Karp
struct TSP {
  n: i32,
  x: Vec<f32>,
  y: Vec<f32>,
  cost: Vec<Vec<f32>>,
  cost_with_pi: Vec<Vec<f32>>,
  best: Node,
}
#[derive(Eq, Default)]
struct Node {
  excluded: Vec<Vec<bool>>,
  pi: Vec<N32>,
  lower_bound: N32,
  degree: Vec<i32>,
  parent: Vec<i32>,
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
  fn exclude(node: Node, i: usize, j: usize) -> Node {
    let mut child: Node = Default::default();
    child.excluded = node.excluded.clone();
    child.excluded[i] = node.excluded[i].clone();
    child.excluded[j] = node.excluded[j].clone();
    child.excluded[i][j] = true;
    child.excluded[j][i] = true;
    //    computeHeldKarp(child);
    child
  }
  fn computeHeldKarp(&self, node: &mut Node) {
    node.lower_bound = n32(std::f32::MIN);
    //    node.degree = new int[n];
    //    node.parent = new int[n];
    let mut lambda = n32(0.1);
    while (lambda > 1e-06) {
      let previous_lower = node.lower_bound;
      //      computeOneTree(node);
      if !(node.lower_bound < self.best.lower_bound) {
        return;
      }
      if !(node.lower_bound < previous_lower) {
        lambda *= 0.9;
      }
      let mut denom: i32 = 0;
      for i in 1..self.n {
        let degree = node.degree[i as usize];
        denom += (degree - 2) ^ 2;
      }
      if (denom == 0) {
        return;
      }
      let t: N32 = lambda * node.lower_bound / n32(denom as f32);
      for i in 1..self.n {
        node.pi[i as usize] += t * n32((node.degree[i as usize] - 2) as f32);
      }
    }
  }
}
//
// fn (*TSP) readInput(Reader r) {
//    //Pattern specification = Pattern.compile("\\s*([A-Z_]+)\\s*(:\\s*([0-9]+))?\\s*");
//    //Pattern data = Pattern.compile("\\s*([0-9]+)\\s+([-+.0-9Ee]+)\\s+([-+.0-9Ee]+)\\s*");
//    let String line ="";
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
//  }
//
//  public void solve() {
//    bestNode.lowerBound = Double.MAX_VALUE;
//    Node currentNode = new Node();
//    currentNode.excluded = new boolean[n][n];
//    costWithPi = new double[n][n];
//    computeHeldKarp(currentNode);
//    PriorityQueue<Node> pq = new PriorityQueue<Node>(11, new NodeComparator());
//    do {
//      do {
//        boolean isTour = true;
//        int i = -1;
//        for (int j = 0; j < n; j++) {
//          if (currentNode.degree[j] > 2 && (i < 0 || currentNode.degree[j] < currentNode.degree[i])) i = j;
//        }
//        if (i < 0) {
//          if (currentNode.lowerBound < bestNode.lowerBound) {
//            bestNode = currentNode;
//            System.err.printf("%.0f", bestNode.lowerBound);
//          }
//          break;
//        }
//        System.err.printf(".");
//        PriorityQueue<Node> children = new PriorityQueue<Node>(11, new NodeComparator());
//        children.add(exclude(currentNode, i, currentNode.parent[i]));
//        for (int j = 0; j < n; j++) {
//          if (currentNode.parent[j] == i) children.add(exclude(currentNode, i, j));
//        }
//        currentNode = children.poll();
//        pq.addAll(children);
//      } while (currentNode.lowerBound < bestNode.lowerBound);
//      System.err.printf("%n");
//      currentNode = pq.poll();
//    } while (currentNode != null && currentNode.lowerBound < bestNode.lowerBound);
//    // output suitable for gnuplot
//    // set style data vector
//    System.out.printf("# %.0f%n", bestNode.lowerBound);
//    int j = 0;
//    do {
//      int i = bestNode.parent[j];
//      System.out.printf("%f\t%f\t%f\t%f%n", x[j], y[j], x[i] - x[j], y[i] - y[j]);
//      j = i;
//    } while (j != 0);
//  }
//
//
//  private void computeOneTree(Node node) {
//    // compute adjusted costs
//    node.lowerBound = 0.0;
//    Arrays.fill(node.degree, 0);
//    for (int i = 0; i < n; i++) {
//      for (int j = 0; j < n; j++) costWithPi[i][j] = node.excluded[i][j] ? Double.MAX_VALUE : cost[i][j] + node.pi[i] + node.pi[j];
//    }
//    int firstNeighbor;
//    int secondNeighbor;
//    // find the two cheapest edges from 0
//    if (costWithPi[0][2] < costWithPi[0][1]) {
//      firstNeighbor = 2;
//      secondNeighbor = 1;
//    } else {
//      firstNeighbor = 1;
//      secondNeighbor = 2;
//    }
//    for (int j = 3; j < n; j++) {
//      if (costWithPi[0][j] < costWithPi[0][secondNeighbor]) {
//        if (costWithPi[0][j] < costWithPi[0][firstNeighbor]) {
//          secondNeighbor = firstNeighbor;
//          firstNeighbor = j;
//        } else {
//          secondNeighbor = j;
//        }
//      }
//    }
//    addEdge(node, 0, firstNeighbor);
//    Arrays.fill(node.parent, firstNeighbor);
//    node.parent[firstNeighbor] = 0;
//    // compute the minimum spanning tree on nodes 1..n-1
//    double[] minCost = costWithPi[firstNeighbor].clone();
//    for (int k = 2; k < n; k++) {
//      int i;
//      for (i = 1; i < n; i++) {
//        if (node.degree[i] == 0) break;
//      }
//      for (int j = i + 1; j < n; j++) {
//        if (node.degree[j] == 0 && minCost[j] < minCost[i]) i = j;
//      }
//      addEdge(node, node.parent[i], i);
//      for (int j = 1; j < n; j++) {
//        if (node.degree[j] == 0 && costWithPi[i][j] < minCost[j]) {
//          minCost[j] = costWithPi[i][j];
//          node.parent[j] = i;
//        }
//      }
//    }
//    addEdge(node, 0, secondNeighbor);
//    node.parent[0] = secondNeighbor;
//    node.lowerBound = Math.rint(node.lowerBound);
//  }
//}
//
