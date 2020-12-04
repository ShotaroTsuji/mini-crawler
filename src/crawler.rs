use std::collections::{VecDeque, HashSet};
use std::borrow::Borrow;
use std::hash::Hash;

pub struct Crawler<'a, G: AdjacentNodes> {
    graph: &'a G,
    visit: VecDeque<<G as AdjacentNodes>::Node>,
    visited: HashSet<<G as AdjacentNodes>::Node>,
}

impl<'a, G> Crawler<'a, G>
where
    G: AdjacentNodes,
    <G as AdjacentNodes>::Node: Clone + Hash + Eq + Borrow<<G as AdjacentNodes>::Node>,
{
    pub fn new(graph: &'a G, start: <G as AdjacentNodes>::Node) -> Self {
        let mut visit = VecDeque::new();
        let visited = HashSet::new();

        visit.push_back(start);

        Self {
            graph: graph,
            visit: visit,
            visited: visited,
        }
    }
}

impl<'a, G> Iterator for Crawler<'a, G>
where
    G: AdjacentNodes,
    <G as AdjacentNodes>::Node: Clone + Hash + Eq + Borrow<<G as AdjacentNodes>::Node>,
{
    type Item = <G as AdjacentNodes>::Node;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(v) = self.visit.pop_front() {
            if self.visited.contains(&v) {
                continue;
            }

            let adj = self.graph.adjacent_nodes(&v);
            for u in adj.into_iter() {
                if !self.visited.contains(&u) {
                    self.visit.push_back(u);
                }
            }

            self.visited.insert(v.clone());

            return Some(v);
        }

        None
    }
}

pub trait AdjacentNodes {
    type Node;

    fn adjacent_nodes(&self, v: &Self::Node) -> Vec<Self::Node>;
}

#[cfg(test)]
mod test {
    use super::*;

    struct AdjVec(Vec<Vec<usize>>);

    impl AdjacentNodes for AdjVec {
        type Node = usize;

        fn adjacent_nodes(&self, v: &Self::Node) -> Vec<Self::Node> {
            self.0.get(*v)
                .cloned()
                .unwrap_or(Vec::new())
        }
    }

    use std::rc::Rc;
    struct RcAdjVec(Vec<Vec<Rc<usize>>>);

    impl AdjacentNodes for RcAdjVec {
        type Node = Rc<usize>;

        fn adjacent_nodes(&self, v: &Self::Node) -> Vec<Self::Node> {
            let v: usize = *v.borrow();
            self.0.get(v)
                .cloned()
                .unwrap_or(Vec::new())
        }
    }

    #[test]
    fn adjvec() {
        let graph = AdjVec(vec![
            vec![1, 2],
            vec![0, 3],
            vec![3],
            vec![2, 0]
        ]);

        assert_eq!(graph.adjacent_nodes(&3), vec![2, 0]);
        assert_eq!(graph.adjacent_nodes(&10), vec![]);
    }

    #[test]
    fn bfs0() {
        let graph = AdjVec(vec![
            vec![1, 2],
            vec![0, 3],
            vec![3],
            vec![2, 0]
        ]);

        let crawler = Crawler::new(&graph, 0);
        let nodes: Vec<usize> = crawler.collect();

        assert_eq!(nodes, vec![0, 1, 2, 3]);
    }

    #[test]
    fn bfs1() {
        let graph = AdjVec(vec![
            vec![1],
            vec![0, 2, 4],
            vec![0, 3],
            vec![0],
            vec![0],
        ]);

        let crawler = Crawler::new(&graph, 0);
        let nodes: Vec<usize> = crawler.collect();

        assert_eq!(nodes, vec![0, 1, 2, 4, 3]);
    }

    #[test]
    fn bfs3() {
        let graph = AdjVec(vec![
            vec![1, 1, 2],
            vec![2, 3],
            vec![],
            vec![]
        ]);

        let crawler = Crawler::new(&graph, 0);
        let nodes: Vec<usize> = crawler.collect();

        assert_eq!(nodes, vec![0, 1, 2, 3]);
    }

    #[test]
    fn rc_bfs() {
        let v0 = Rc::new(0);
        let v1 = Rc::new(1);
        let v2 = Rc::new(2);
        let v3 = Rc::new(3);
        let graph = RcAdjVec(vec![
            vec![v1.clone(), v2.clone()],
            vec![v0.clone(), v3.clone()],
            vec![v3.clone()],
            vec![v2.clone(), v0.clone()],
        ]);

        let crawler = Crawler::new(&graph, v0.clone());
        let nodes: Vec<Rc<usize>> = crawler.collect();

        assert_eq!(nodes, vec![v0, v1, v2, v3]);
    }
}
