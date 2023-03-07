fn clear_mat (x: usize, y: usize, size : usize, mat: &mut Vec<Vec<bool>>) {
    for j in y..size {
        mat[x][j] = false;
    }

    for i in (x+1)..size {
        for j in 0..size {
            mat[i][j] = false;
        }
    }
}
fn can_put(x: usize, y: usize, size : usize, mat: &mut Vec<Vec<bool>>) -> bool {
    if mat[x][y] {
        return false;
    }
    if (y > 0 && mat[x][y - 1]) || (y + 1 < size && mat[x][y + 1])  {
        return false;
    }
    if (x > 0 && mat[x - 1][y]) || (x + 1 < size && mat[x + 1][y])  {
        return false; 
    }
    return true;
}

fn print_mat (size : usize, mat: &mut Vec<Vec<bool>>) {
    for i in 0..size {
        for j in 0..size {
            print!("{} ", mat[i][j] as i8);
        }
        println!();
    }
    println!();
}

fn put (x: usize, y: usize, mat: &mut Vec<Vec<bool>>, size : usize, suma : &mut usize) {

    if can_put(x, y, size, mat) {
        *suma = *suma + 1;
        mat[x][y] = true;
        if y + 1 < size {
            put(x,y+1,mat,size,suma);
        }
        else if x + 1 < size {
            put(x+1,0,mat,size,suma);
        }
        else {
            return;
        }
    }

    mat[x][y] = false;
    clear_mat(x, y, size, mat);
    if y + 1 < size {
        put(x,y+1,mat,size,suma);
    }
    else if x + 1 < size {
        put(x+1,0,mat,size,suma);
    }
    else {
        return;
    }
    
}

fn main() {
    let size = 8;
    let mut suma: usize ;
    suma = 1;
    let mat = & mut vec![vec![false; size]; size];
    put (0,0,mat,size, &mut suma);
    println!("Suma = {}", suma);
}
