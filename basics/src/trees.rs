#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct BinTreeNode<T> {
    value: T,
    previous_node: Option<Box<BinTreeNode<T>>>,
}

/// Levels start at... 1 makes sense 2^1-1, 2^3-1
pub struct BinTreeVec<T>
where
    T: Sized,
{
    levels: u32,
    pos: usize,
    tree: Vec<Option<T>>,
}

impl<T> BinTreeVec<T>
where
    T: Sized + std::marker::Copy,
{
    pub fn new(levels: u32) -> Self {
        let num_of_elms: u32 = Self::calc_total_elms(levels).unwrap();
        Self {
            levels: levels,
            pos: 0,
            tree: vec![None::<T>; num_of_elms as usize],
        }
    }

    fn calc_total_elms(levels: u32) -> Result<u32, String> {
        if levels < 1 {
            Err(String::from("Levels must be greater than 0."))
        } else {
            let base: u32 = 2;
            Ok(base.pow(levels) - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// 1--------------o----------------
    /// 2------------o---o--------------
    /// 3-----------o-o-o-o-------------
    fn create_new_bintreevec() {
        let actual = BinTreeVec::<i32>::new(3);
        assert_eq!(actual.levels, 3);
        assert_eq!(actual.pos, 0);
        assert_eq!(actual.tree.capacity(), 7);
        assert_eq!(actual.tree[0], None);
    }
}
