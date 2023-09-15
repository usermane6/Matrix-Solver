#[derive(Clone)]
pub struct Matrix {
    size: (usize, usize),
    vals: Vec<f64>,
}

impl Matrix {
    //! SIZE IS WIDTH TIMES HEIGHT, NOT HEIGHT TIMES WIDTH. MATHEMATICIANS ARE JUST DUMB
    pub fn new(size: (usize, usize), vals: Vec<f64>) -> Self { Self { size, vals } }

    
    /// height of matrix
    pub fn h(&self) -> usize { self.size.1 }
    
    /// width of matrix
    pub fn w(&self) -> usize { self.size.0 }
    
    pub fn get(&self, x:usize, y:usize) -> &f64 {
        let id = (self.size.0 * y) + x;

        if let Some(val) = self.vals.get(id) {
            val
        } else {
            panic!("\n\nCannot get at ({}, {}) out of range \nSize of matrix is ({}, {}) \nID is {}\n\n", x, y, self.size.0, self.size.1, id)
        }
    }

    pub fn get_id(&self, id:usize) -> &f64 {
        if let Some(val) = self.vals.get(id) {
            val
        } else {
            panic!("\n\nCannot get at id {}, out of range \nLength of matrix is {}\n\n", id, self.vals.len())
        }
    }

    pub fn set(&mut self, x:usize, y:usize, new_val: f64) {
        let id = (self.size.0 * y) + x;

        if let Some(val) = self.vals.get(id) {
            self.vals[id] = new_val;
        } else {
            panic!("\n\nGiven point ({}, {}) out of range \nSize of matrix is ({}, {})\n\n", x, y, self.size.0, self.size.1)
        }
    }

    pub fn set_id(&mut self, id:usize, new_val: f64) { // sets by id rather than coordinate
        if let Some(val) = self.vals.get(id) {
            self.vals[id] = new_val;
        } else {
            panic!("\n\nCannot get at id {}, out of range \nLength of matrix is {}\n\n", id, self.vals.len())
        }
    }

    /// performs the function of:
    /// k * R_r => R_r
    /// 
    /// # Arguments
    /// - `k` is any constant
    /// - `r` is a row number
    pub fn multiply_assign (self, k: f64, r: usize) -> Matrix {
        let mut new_matrix = self.clone();
        let start_id = r * self.size.0;
        
        for id in start_id..(start_id + self.size.0) {
            new_matrix.set_id(id, self.get_id(id) * k);
        }

        new_matrix
    }

    /// performs the function of:
    /// k * R_r_1 + R_r_2 => R_r_2
    /// 
    /// # Arguments
    /// - `k` is any constant
    /// - `r_1` and `r_2` are a row numbers
    pub fn multiply_add_assign (self, k: f64, r_1:usize, r_2:usize) -> Matrix {
        let mut new_matrix = self.clone();

        let id_1 = r_1 * self.size.0;
        let id_2 = r_2 * self.size.0;

        for offset in 0..self.size.0 {
            let new_val = ( self.get_id( id_1 + offset ) * k ) + self.get_id( id_2 + offset );  // ( R_r_1 * k ) + R_r_2 => R_r_2
            new_matrix.set_id( id_2 + offset, new_val );
        }

        new_matrix
    }   

    /// simply swaps two rows
    /// 
    /// # Arguments
    ///  - `r_1` and `r_2` are a row numbers
    pub fn switch_rows (self, r_1:usize, r_2:usize) -> Matrix {
        let mut new_matrix = self.clone();

        let id_1 = r_1 * self.size.0;
        let id_2 = r_2 * self.size.0;

        for offset in 0..self.size.0 {
            let val_1 = *self.get_id(id_2 + offset);
            new_matrix.set_id(id_1 + offset, val_1);
            
            let val_2 = *self.get_id(id_1 + offset);
            new_matrix.set_id(id_2 + offset, val_2);
        }

        new_matrix
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // formatting of matrix into strings so they can be printed to the console

        let mut final_string = String::new();

        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let mut val = self.get(x, y).to_string();
                let mut gap = "      ".to_string();

                if val.len() > gap.len() - 1 { // minus 1 so that there is a space between numbers, even at max length
                    val.replace_range(5..val.len(), ""); // truncate numbers, so the don't upset alignments
                }


                gap.replace_range(0..val.len(), ""); // adjust gap size between #s so #s are aligned
                
                final_string.push_str(gap.as_str());
                final_string.push_str(val.as_str());

            }

            final_string.push_str("\n")
        }

        write!(f, "{}", final_string)
    }
}