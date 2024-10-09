use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone)]
struct Vertex<T> {
    key: T,
    neighbors: Vec<(T, i32)>,
}

impl<T: Clone + PartialEq> Vertex<T> {
    fn new(key: T) -> Self {
        Self {
            key,
            neighbors: Vec::new(),
        }
    }

    fn adjacent_key(&self, key: &T) -> bool {
        for (nbr, _wt) in self.neighbors.iter() {
            if nbr == key {
                return true;
            }
        }
        false
    }

    fn add_neighbor(&mut self, nbr: T, wt: i32) {
        self.neighbors.push((nbr, wt));
    }

    fn get_neighbors(&self) -> Vec<&T> {
        let mut nbrs = Vec::new();
        for (nbr, _wt) in self.neighbors.iter() {
            nbrs.push(nbr);
        }
        nbrs
    }

    fn get_nbr_weight(&self, key: &T) -> &i32 {
        for (nbr, wt) in self.neighbors.iter() {
            if nbr == key {
                return wt;
            }
        }
        &0
    }
}

#[derive(Debug)]
struct Graph<T> {
    vertnums: u32,
    edgenums: u32,
    vertics: HashMap<T, Vertex<T>>,
}

impl<T: Hash + Eq + PartialEq + Clone> Graph<T> {
    fn new() -> Self {
        Self {
            vertnums: 0,
            edgenums: 0,
            vertics: HashMap::<T, Vertex<T>>::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.vertnums == 0
    }
    fn vertex_num(&self) -> u32 {
        self.vertnums
    }
    fn edge_num(&self) -> u32 {
        self.edgenums
    }

    fn contains(&self, key: &T) -> bool {
        for (nbr, _vertex) in self.vertics.iter() {
            if nbr == key {
                return true;
            }
        }
        false
    }

    fn add_vertex(&mut self, key: &T) -> Option<Vertex<T>> {
        let vertex = Vertex::new(key.clone());
        self.vertnums += 1;
        self.vertics.insert(key.clone(), vertex)
    }

    fn get_vertex(&self, key: &T) -> Option<&Vertex<T>> {
        if let Some(ver) = self.vertics.get(key) {
            Some(&ver)
        } else {
            None
        }
    }

    fn vertex_keys(&self) -> Vec<T> {
        let mut keys = Vec::new();
        for key in self.vertics.keys() {
            keys.push(key.clone());
        }
        keys
    }

    fn remove_vertex(&mut self, key: &T) -> Option<Vertex<T>> {
        let old_vertex = self.vertics.remove(key);
        self.vertnums -= 1;
        self.edgenums -= old_vertex.clone().unwrap().get_neighbors().len() as u32;

        for v in self.vertex_keys() {
            if let Some(vt) = self.vertics.get_mut(&v) {
                if vt.adjacent_key(key) {
                    vt.neighbors.retain(|(k, _)| k != key);
                    self.edgenums -= 1;
                }
            }
        }
        old_vertex
    }

    fn add_edge(&mut self, from: &T, to: &T, wt: i32) {
        if !self.contains(from) {
            self.add_vertex(from);
        }
        if !self.contains(to) {
            self.add_vertex(to);
        }
        self.edgenums += 1;
        self.vertics
            .get_mut(from)
            .unwrap()
            .add_neighbor(to.clone(), wt);
    }

    fn adjacent(&self, from: &T, to: &T) -> bool {
        self.vertics.get(from).unwrap().adjacent_key(to)
    }
}

fn main() {
    let mut g = Graph::new();
    for i in 0..6 {
        g.add_vertex(&i);
    }

    let vertics = g.vertex_keys();
    println!("vertics keys:{:#?}", vertics);

    g.add_edge(&0, &1, 5);
    g.add_edge(&0, &5, 2);
    g.add_edge(&1, &2, 4);
    g.add_edge(&2, &3, 9);
    g.add_edge(&3, &4, 7);
    g.add_edge(&3, &5, 3);
    g.add_edge(&4, &0, 1);
    g.add_edge(&4, &4, 8);

    println!("vert nums:{}", g.vertex_num());
    println!("edge nums:{}", g.edge_num());
    println!("contains 0:{}", g.contains(&0));
    let vet = g.get_vertex(&0).unwrap();

    println!("key:{},to nbr 1 weight:{}", vet.key, vet.get_nbr_weight(&1));

    let keys = vet.get_neighbors();
    for nbr in keys {
        println!("nbr:{nbr}");
    }

    for (nbr, wt) in vet.neighbors.iter() {
        println!("0 neighbor :{nbr} wt:{wt}");
    }

    let res = g.adjacent(&0, &1);
    println!("0 adjacent to 1:{res}");

    let res = g.adjacent(&3, &2);
    println!("3 adjacent to 2:{res}");

    let rm = g.remove_vertex(&0).unwrap();
    println!("remove vertex:{}", rm.key);
    println!("left vert nums:{}", g.vertex_num());
    println!("left edge nums:{}", g.edge_num());
    println!("contains 0: {}", g.contains(&0));
}
