fn parse_forest(input: impl Iterator<Item = String>) -> Vec<Vec<u8>> {
    input
        .map(|s| s.bytes().map(|c| c - '0' as u8).collect())
        .collect()
}

#[derive(Debug)]
struct TreeRef<'a> {
    forest: &'a Vec<Vec<u8>>,
    i: usize,
    j: usize,
}

impl<'a> TreeRef<'a> {
    fn directions(
        &self,
    ) -> (
        impl Iterator<Item = (usize, usize)>,
        impl Iterator<Item = (usize, usize)>,
        impl Iterator<Item = (usize, usize)>,
        impl Iterator<Item = (usize, usize)>,
    ) {
        let TreeRef { forest, i, j } = *self;
        (
            (0..j).map(move |j| (i, j)).rev(),
            (j + 1..forest[i].len()).map(move |j| (i, j)),
            (0..i).map(move |i| (i, j)).rev(),
            (i + 1..forest.len()).map(move |i| (i, j)),
        )
    }

    fn visible(&self) -> bool {
        let (mut left, mut right, mut top, mut bottom) = self.directions();
        let TreeRef { forest, i, j } = *self;
        let less_height = |(i2, j2): (usize, usize)| forest[i2][j2] < forest[i][j];
        left.all(less_height)
            || right.all(less_height)
            || top.all(less_height)
            || bottom.all(less_height)
    }

    fn count_visible_trees(&self, direction: impl Iterator<Item = (usize, usize)>) -> u64 {
        let mut count = 0;
        for (i2, j2) in direction {
            debug_assert_ne!((i2, j2), (self.i, self.j));
            count += 1;
            if self.forest[i2][j2] >= self.forest[self.i][self.j] {
                break;
            }
        }
        count
    }

    fn scenic_score(&self) -> u64 {
        let (left, right, top, bottom) = self.directions();
        [
            self.count_visible_trees(left),
            self.count_visible_trees(right),
            self.count_visible_trees(top),
            self.count_visible_trees(bottom),
        ]
        .iter()
        .product()
    }
}

pub fn solve1(input: Vec<String>) -> usize {
    let forest = &parse_forest(input.into_iter());
    let exterior_visible = 2 * forest.len() + 2 * forest[0].len() - 4;
    let interior_visible = (1..forest.len() - 1)
        .flat_map(|i| (1..forest[0].len() - 1).map(move |j| (i, j)))
        .filter(|(i, j)| {
            TreeRef {
                forest,
                i: *i,
                j: *j,
            }
            .visible()
        })
        .count();
    exterior_visible + interior_visible
}

pub fn solve2(input: Vec<String>) -> u64 {
    let forest = &parse_forest(input.into_iter());
    let tree_idx =
        (1..forest.len() - 1).flat_map(|i| (1..forest[0].len() - 1).map(move |j| (i, j)));
    tree_idx
        .map(|(i, j)| TreeRef { forest, i, j }.scenic_score())
        .max()
        .unwrap()
}
