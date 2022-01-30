pub struct DisjointSet {
    parent: Vec<usize>,
}
impl DisjointSet {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }

    pub fn find(self: &mut Self, a: usize) -> usize {
        if self.parent[a] == a {
            return a;
        }
        self.parent[a] = self.find(self.parent[a]);
        self.parent[a]
    }

    pub fn merge(&mut self, a: usize, b: usize) -> bool {
        let a = self.find(a);
        let b = self.find(b);

        if a == b {
            return false;
        }
        self.parent[a] = b;
        true
    }
}
