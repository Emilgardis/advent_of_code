use itertools::Itertools;
pub trait ArrayTools {
    type Item;

    fn disjoint_mut<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> Result<[&mut Self::Item; N], eyre::Report>;
}

impl<T> ArrayTools for [T] {
    type Item = T;

    /// Get multiple mutable references to elements in an array
    fn disjoint_mut<const N: usize>(
        &mut self,
        indices: [usize; N],
    ) -> Result<[&mut Self::Item; N], eyre::Report> {
        if !indices.iter().all_unique() {
            eyre::bail!("entries must be unique, found duplicate")
        }
        if !indices.iter().all(|&i| i < self.len()) {
            eyre::bail!("entries must be unique, found duplicate")
        }

        let ptr = self.as_mut_ptr();
        // Safety:
        // This is safe because we index the pointer having checked that
        // 1. The indices are unique, so there is only one mutable reference for each index
        // 2. The indices are within range, so the pointer arithmetic will not overflow
        // 3. The slice is not empty, so the pointer is not null
        let arr: [_; N] = indices.map(|i| unsafe { &mut *ptr.add(i) });

        Ok(arr)
    }
}

#[cfg(test)]
#[test]
fn test_disjoint_mut() {
    let mut v = vec![1, 2, 3, 4, 5];

    // Test error when provided with duplicate indices
    let result = v.disjoint_mut([1, 2, 1]).unwrap_err();
    assert_eq!(
        result.to_string(),
        "entries must be unique, found duplicate"
    );

    // Test error when provided with indices that are out of bounds
    let result = v.disjoint_mut([1, 2, 6]).unwrap_err();
    assert_eq!(
        result.to_string(),
        "entries must be unique, found duplicate"
    );

    // Test valid indices
    let result = v.disjoint_mut([1, 2, 3]).unwrap();
    assert_eq!(result, [&mut 2, &mut 3, &mut 4]);

    // Test ascending order indices
    let result = v.disjoint_mut([0, 1, 2]).unwrap();
    assert_eq!(result, [&mut 1, &mut 2, &mut 3]);

    // Test descending order indices
    let result = v.disjoint_mut([2, 1, 0]).unwrap();
    assert_eq!(result, [&mut 3, &mut 2, &mut 1]);

    // Test single index
    let result = v.disjoint_mut([1]).unwrap();
    assert_eq!(result, [&mut 2]);

    // Test maximum allowed number of indices
    let result = v.disjoint_mut([0, 1, 2, 3, 4]).unwrap();
    assert_eq!(result, [&mut 1, &mut 2, &mut 3, &mut 4, &mut 5]);
}
