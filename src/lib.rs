use std::fmt::{Debug, Formatter, self};

pub struct Matrix {
    ptr: *const i32,
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
    pub fn new(m: &[&[i32]]) -> Self {
        assert!(m.iter().all(|e| e.len() == m.len()), "Matrix must be square");
        let x = unsafe { (*m.as_ptr()).as_ptr() };
        Self {
            ptr: x,
            side_length: m.len(),
        }
    }

    //SAFETY: Must ensure that all calls to at are disjoint.
    pub unsafe fn at(&mut self, i: usize, j: usize) -> &mut i32 {
        &mut *(self.ptr
            .offset((i * self.side_length) as isize)
            .offset(j as isize) as *mut i32)
    }
}


#[cfg(test)]
mod tests {
    use crate::Matrix;

    #[test]
    fn vec_as_ptr() {
        let row1: [i32; 2] = [1, 2];
        let row2: [i32; 2] = [3, 4];
        let mat: [&[i32]; 2] = [&row1, &row2];
        let mut m = Matrix::new(&mat);
        unsafe {
            assert_eq!(*m.at(1, 1), 4);
            dbg!(&m);
            assert_eq!(*m.at(1, 1), 4);
        }
    }
}
