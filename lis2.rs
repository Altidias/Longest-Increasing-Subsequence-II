pub struct SegmentTree<T> {
    n: usize,
    tree: Vec<T>,
}

impl<T> SegmentTree<T>
where
    T: Copy + Ord + Default,
{
    pub fn new(nums: &[T]) -> Self {
        let n = nums.len();
        let mut tree: Vec<_> = vec![T::default(); 2 * n];
        for i in n..(2 * n) {
            tree[i] = nums[i - n];
        }
        for i in (1..n).rev() {
            tree[i] = tree[2 * i].max(tree[2 * i + 1]);
        }
        Self { n, tree }
    }

    pub fn update(&mut self, index: usize, val: T) {
        let mut i = index + self.n;
        self.tree[i] = val;
        while i > 0 {
            let (child1, child2) = (i, if i % 2 == 0 { i + 1 } else { i - 1 });
            i /= 2;
            self.tree[i] = self.tree[child1].max(self.tree[child2]);
        }
    }

    pub fn max_range(&self, left: usize, right: usize) -> T {
        let (mut left, mut right) = (left + self.n, right + self.n);
        let mut max = T::default();
        while left <= right {
            if left % 2 == 1 {
                max = max.max(self.tree[left]);
                left += 1;
            }
            if right % 2 == 0 {
                max = max.max(self.tree[right]);
                right -= 1;
            }
            left /= 2;
            right /= 2;
        }
        max
    }
}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut dp = SegmentTree::new(&[0; 100002]);
        let mut global_max = 1;
        for i in 0..n {
            let (low, high) = (0.max(nums[i] - k) as usize, 0.max(nums[i] - 1) as usize);
            let max = dp.max_range(low, high) + 1;
            global_max = global_max.max(max);
            dp.update(nums[i] as usize, max);
        }
        global_max
    }
}