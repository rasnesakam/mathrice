use crate::types::Matrix;
use std::fmt;

impl<const R: usize, const C: usize, T: Sized + Copy + fmt::Display> fmt::Display for Matrix<R,C,T>{

    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut content: String = String::new();
        for i in 0..self.row_size {
            content.push('[');
            for j in 0..self.col_size {
                content.push_str(format!("{}",self[i][j]).as_str());
                if j < self.col_size - 1 {
                    content.push_str(", ");
                }
            }
            content.push_str("]\n");
        }
        write!(fmt,"{}",content.to_string())
    }

}

impl <const ROW: usize, const COL: usize, T> Matrix<ROW, COL, T>
	where T: Sized + Copy + fmt::Display{
	pub fn print(&self) {
		for i in 0..self.row_size {
			print!("[ ");
			for j in 0..self.col_size {
				print!("{}",self.data[i][j]);
				if j < self.col_size - 1 {
					print!(", ")
				}
			}
			println!(" ]");
		}
	}
}