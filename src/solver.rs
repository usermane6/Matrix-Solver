use crate::matrix::Matrix;

enum Zeros {
    YES,
    NO(usize, usize)
}

fn check_col_for_vals (m: &Matrix, col: usize, row: usize) -> Zeros { // checks if the row has nonzero values, will check downwards from the row param
    for y in row..m.h() {
        if *m.get(col, y) != 0.0 { return Zeros::NO(col, y) } // returns the first position of a value
    }
    return Zeros::YES;
}

fn reduce_below(mut m: Matrix, col: usize, pivot_row: usize) -> Matrix { // gets all values below the pivot to be 0
    
    let m_copy = m.clone();
    let comp_val = *m_copy.get(col, pivot_row); // the value that all rows will be compared to
 
    for y in pivot_row + 1..m.h() {
        let k = -( *m_copy.get(col, y) / comp_val ); // get constant value needed to multiply the pivot row by, so that when it is added to another row, that row will be 0 

        m = m.clone().multiply_add_assign(k, pivot_row, y);

        println!("{}", m)
    }

    m
}

fn reduce_above(mut m: Matrix, col: usize, pivot_row: usize) -> Matrix { // gets all values below the pivot to be 0
    
    let m_copy = m.clone();
    let comp_val = *m_copy.get(col, pivot_row); // the value that all rows will be compared to
 
    for y in (0..pivot_row).rev() {
        let k = -( *m_copy.get(col, y) / comp_val ); // get constant value needed to multiply the pivot row by, so that when it is added to another row, that row will be 0 

        m = m.clone().multiply_add_assign(k, pivot_row, y);

        println!("{}", m)
    }

    m
}

fn prettify_row(m: Matrix, col: usize, row: usize) -> Matrix {
    let m_copy = m.clone();
    m.multiply_assign(1.0 / m_copy.get(col, row), row)
}

pub fn row_reduce(mut m: Matrix) -> Matrix{
    // let h = m.h(); 
    let w = m.w();

    let mut pivots: Vec<(usize, usize)> = vec![];

    for x in 0..w - 1 { // minus 1 is assuming we have an augmented matrix, last col is solutions

        // search for nonzero column
        if let Zeros::NO(_, y) = check_col_for_vals(&m, x, pivots.len()) { 
            if y != 0 { m = m.switch_rows(pivots.len(), y)} // swap row to highest without a pivot

            pivots.push((x, y));

            m = reduce_below(m, x, y);
        }
    }
    
    for (x, y) in pivots.into_iter().rev() {
        m = reduce_above(m, x, y);
        m = prettify_row(m, x, y)
    }

    m
}