struct UnionSet {
  father: Vec<usize>,
  rank: Vec<usize>
}

impl UnionSet {
  fn new(size: usize) -> Self {
    UnionSet {
      father: {
        let mut _father: Vec<usize> = vec![0; size];
        for i in 0..size {
          _father[i] = i;
        }
        _father
      },
      rank: vec![1; size]
    }
  }
  fn find(&mut self, index: usize) -> usize {
    if index == self.father[index] {
      index
    } else {
      self.father[index] = find(self.father[index]);
      self.father[index]
    }
  }
  fn merge(&mut self, a: usize, b: usize) -> () {
    let (fa, fb) = (find(a), find(b));
    if self.rank[fa] < self.rank[fb] {
      self.father[fa] = fb;
    } else {
      self.father[fb] = fa;
    }
    if self.rank[fa] == self.rank[fb] && fa != fb {
      self.rank[fa] += 1;
    }
  }
}