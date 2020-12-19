use std::fmt::{Debug, Formatter, self};
use std::ops::{Index, IndexMut};
use std::mem;

pub struct Matrix {
    ptr: *mut i32,
    side_length: usize,
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Now _this_ is pod racing!
        let vec_vec2 = unsafe {
            std::slice::from_raw_parts(self.ptr, self.side_length * self.side_length)
                .chunks_exact(self.side_length)
                .collect::<Vec<&[i32]>>()
                .iter()
                .map(|&slice| Vec::from(slice))
                .collect::<Vec<Vec<i32>>>()
        };

        f.debug_struct("Matrix")
            .field("ptr", &vec_vec2)
            .field("side_length", &self.side_length)
            .finish()
    }
}


impl Matrix {
    pub fn new(m: Vec<Vec<i32>>) -> Self {
        assert!(m.iter().all(|e| e.len() == m.len()), "Matrix must be square");
        let len = m.len();
        let flat_vec = m.into_iter().flatten().collect::<Vec<i32>>();
        let ptr = flat_vec.as_ptr() as *mut _;
        // This pointer own the vec now.
        mem::forget(flat_vec);
        Self {
            ptr,
            side_length: len,
        }
    }

    //SAFETY: Must ensure that all calls to at are disjoint.
    pub unsafe fn at(&mut self, i: usize, j: usize) -> &mut i32 {
        &mut *(self.ptr
            .offset((i * self.side_length) as isize)
            .offset(j as isize) as *mut i32)
    }
}

impl Index<usize> for Matrix {
    type Output = [i32];

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { std::slice::from_raw_parts(self.ptr.offset((index * self.side_length) as isize), self.side_length) }
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut [i32] {
        unsafe { std::slice::from_raw_parts_mut((self.ptr as *mut i32).offset((index * self.side_length) as isize), self.side_length) }
    }
}

impl Drop for Matrix {
    fn drop(&mut self) {
        let total_length = self.side_length * self.side_length;
        let upper_bound_power_2 = |mut n: usize| {
            let mut count = 0;
            if n != 0 && (n & (n - 1)) == 0 {
                n
            } else {
                while n != 0 {
                    n >>= 1;
                    count += 1;
                }
                1 << count
            }
        };
        let cap = upper_bound_power_2(total_length);
        let _ = unsafe { std::vec::Vec::from_raw_parts(self.ptr, total_length, cap) };
    }
}

#[cfg(test)]
mod tests {
    use crate::Matrix;

    #[test]
    fn indexing() {
        let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let mut m = Matrix::new(v);
        assert_eq!(2, m[0][1]);
        m[1][1] = 1;
        dbg!(m);
    }
}
