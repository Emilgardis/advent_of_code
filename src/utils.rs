use itertools::Itertools;

pub trait ArrayTools<T> {
    fn disjoint_mut<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> Result<[&mut T; N], eyre::Report>;
}

impl<T> ArrayTools<T> for [T] {
    /// Get multiple mutable references to elements in an array
    fn disjoint_mut<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> Result<[&mut T; N], eyre::Report> {
        self.get_many_mut(indices).map_err(Into::into)
    }
}

/// Takes indices into e.g a `Vec<Vec<T>>` and returns them mutably
///
/// # Examples
///
/// ```rust
/// use aoc::utils::ArrayArrayTools;
///
/// let mut v: Vec<Vec<u8>> = vec![vec![1,2,3,4], vec![4,3,2,1]];
/// let [a, b] = v.double_disjoint_mut([(0,0), (1,0)]).unwrap();
/// std::mem::swap(a,b);
/// assert_eq!(v, vec![vec![4,2,3,4], vec![1,3,2,1]]);
/// ```
pub trait ArrayArrayTools<C, T> {
    fn double_disjoint_mut<const N: usize>(
        &mut self,
        indices: [(usize, usize); N],
    ) -> Result<[&mut T; N], eyre::Report>;
}

impl<C, T> ArrayArrayTools<C, T> for [C]
where
    C: AsMut<[T]> + AsRef<[T]>,
{
    fn double_disjoint_mut<const N: usize>(
        &mut self,
        indices: [(usize, usize); N],
    ) -> Result<[&mut T; N], eyre::Report> {
        fn get_many_check_valid(indices: impl Iterator<Item = usize> + Clone, len: usize) -> bool {
            let mut valid = true;
            for (i, idx) in indices.clone().enumerate() {
                valid &= idx < len;
                for idx2 in indices.clone().take(i) {
                    valid &= idx != idx2;
                }
            }
            valid
        }

        let mut valid = true;
        for &(i, j) in indices.iter().unique_by(|(i, _)| i) {
            let Some(len) = self.get(i).map(|s| s.as_ref().len()) else {
                eyre::bail!("index {i}, {j} out of bound")
            };
            valid &= get_many_check_valid(
                indices
                    .iter()
                    .filter(|(l, _)| i == *l)
                    .map(|(_, k)| k)
                    .copied(),
                len,
            );
        }
        if !valid {
            eyre::bail!("invalid index found")
        }

        let mut arr: std::mem::MaybeUninit<[&mut T; N]> = std::mem::MaybeUninit::uninit();

        // SAFETY: We expect `indices` to contain disjunct values that are
        // in bounds of all slices in self.
        let arr_ptr = arr.as_mut_ptr();
        unsafe {
            for i in 0..N {
                let idx = *indices.get_unchecked(i);

                let slice: *mut [T] = self[idx.0].as_mut();

                *(*arr_ptr).get_unchecked_mut(i) = &mut *slice.get_unchecked_mut(idx.1);
            }
            Ok(arr.assume_init())
        }
    }
}

#[test]
fn test_double_disjoint_mut() {
    let mut v: Vec<Vec<u8>> = vec![vec![1, 2, 3, 4], vec![4, 3, 2, 1]];
    assert!(v.double_disjoint_mut([(0, 0), (0, 1)]).is_ok());
    let [a, b] = v.double_disjoint_mut([(0, 0), (1, 0)]).unwrap();
    std::mem::swap(a, b);
    assert_eq!(v, vec![vec![4, 2, 3, 4], vec![1, 3, 2, 1]]);

    let mut v: Vec<Vec<u8>> = vec![vec![1, 2, 3, 4], vec![4, 3, 2, 1]];
    assert!(v.double_disjoint_mut([(0, 4), (1, 0)]).is_err());
    assert!(v.double_disjoint_mut([(0, 0), (4, 0)]).is_err());
    v.double_disjoint_mut([(0, 0), (0, 3), (1,0), (1,1), (1,2)]).unwrap();
    assert!(v.double_disjoint_mut([(0, 4), (0, 4)]).is_err());
}
