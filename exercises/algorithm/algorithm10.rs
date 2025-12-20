use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }

    /// 添加无向边：需要在两个节点的邻接表中互相记录对方
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (from, to, weight) = edge;
        // 确保两个节点都存在于图中（不存在则添加）
        self.add_node(from);
        self.add_node(to);

        // 向from节点的邻接表中添加to节点和权重
        self.adjacency_table_mutable()
            .get_mut(from)
            .unwrap() // 因为已经add_node，所以一定存在
            .push((to.to_string(), weight));

        // 向to节点的邻接表中添加from节点和权重（无向边的特性）
        self.adjacency_table_mutable()
            .get_mut(to)
            .unwrap() // 因为已经add_node，所以一定存在
            .push((from.to_string(), weight));
    }
}

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;

    /// 添加节点到图中
    /// 返回值：true表示节点是新添加的，false表示节点已存在
    fn add_node(&mut self, node: &str) -> bool {
        // 如果节点已存在，返回false
        if self.contains(node) {
            return false;
        }
        // 节点不存在，添加空的邻接列表并返回true
        self.adjacency_table_mutable()
            .insert(node.to_string(), Vec::new());
        true
    }

    fn add_edge(&mut self, edge: (&str, &str, i32));

    /// 检查节点是否存在于图中
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }

    /// 获取图中所有节点的集合
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }

    /// 获取图中所有边的列表（(起点, 终点, 权重)）
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }

    // 可选：添加测试add_node的用例，验证节点添加逻辑
    #[test]
    fn test_add_node() {
        let mut graph = UndirectedGraph::new();
        // 第一次添加节点a，应该返回true
        assert_eq!(graph.add_node("a"), true);
        // 再次添加节点a，应该返回false
        assert_eq!(graph.add_node("a"), false);
        // 检查节点是否存在
        assert_eq!(graph.contains("a"), true);
        assert_eq!(graph.contains("b"), false);
    }
}
