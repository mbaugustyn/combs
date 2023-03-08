use std::vec;

fn can_put(x: usize, y: usize, size: usize, mat: &mut Vec<Vec<bool>>) -> bool {
    if mat[x][y] {
        return false;
    }
    if (y > 0 && mat[x][y - 1]) || (y + 1 < size && mat[x][y + 1]) {
        return false;
    }
    if (x > 0 && mat[x - 1][y]) || (x + 1 < size && mat[x + 1][y]) {
        return false;
    }
    return true;
}

fn print_mat(size: usize, mat: &mut Vec<Vec<bool>>) {
    for i in 0..size {
        for j in 0..size {
            print!("{} ", mat[i][j] as i8);
        }
        println!();
    }
    println!();
}

// fn put (x: usize, y: usize, mat: &mut Vec<Vec<bool>>, size : usize, suma : &mut usize) {

//     if can_put(x, y, size, mat) {
//         *suma = *suma + 1;
//         mat[x][y] = true;
//         if y + 1 < size {
//             put(x,y+1,mat,size,suma);
//         }
//         else if x + 1 < size {
//             put(x+1,0,mat,size,suma);
//         }
//         else {
//             return;
//         }
//     }

//     mat[x][y] = false;
//     clear_mat(x, y, size, mat);
//     if y + 1 < size {
//         put(x,y+1,mat,size,suma);
//     }
//     else if x + 1 < size {
//         put(x+1,0,mat,size,suma);
//     }
//     else {
//         return;
//     }
// }

// fn combs_recursive (size: usize) -> usize {
//     let mut suma: usize = 1;
//     let mat = & mut vec![vec![false; size]; size];
//     put (0,0,mat,size, &mut suma);
//     return suma;
// }

/*
fn get_max (v : &Vec<usize>) -> usize {

    *v.iter().max().unwrap()
}

fn can_put_nawroty (x : usize, y  : usize, size : usize, mat : &Vec<Vec<usize>>) -> bool {
    if y > 0 && mat[x].contains(&(y-1)) {
        return false;
    }
    if x > 0 && mat[x-1].contains(&y) {
        return false;
    }
    return true;
}


fn combs_nawroty (size: usize) -> usize {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let how_many = 2;
    let mut suma = 0;
    let mat: &mut Vec<Vec<usize>> = & mut vec![vec![]; size];
    mat[0].push(0);

    while x < how_many {
        y = 0;
        while (y < size && !can_put_nawroty(x,y,size, mat)) {
            y += 1;
        }

        if y < size {
            mat[x].push(y);
            suma += 1;
            x += 1;
        }
        else {
            mat[x] = vec![];
            if x == 0 {return suma};
            x -= 1;
        }

    }
    return suma;
}
 */
fn atleast1(size: usize, mat: &Vec<bool>) -> bool {
    for y in mat {
        if *y == true {
            return true;
        }
    }
    return false;
}

fn highest(size: usize, mat: &Vec<bool>) -> usize {
    let mut max = 0;
    for i in 0..size {
        if mat[i] == true {
            max = i;
        }
    }
    return max;
}

fn how_many_1s(size: usize, mat: &Vec<bool>) -> usize {
    let mut suma = 0;
    for i in 0..size {
        if mat[i] == true {
            suma += 1;
        }
    }
    return suma;
}

fn already_inplace(size: usize, mat: &Vec<Vec<bool>>) -> usize {
    let mut suma = 0;
    for i in 0..size {
        for j in 0..size {
            suma += mat[i][j] as usize;
        }
    }
    return suma;
}

fn first_1_row(size: usize, mat: &Vec<Vec<bool>>) -> usize {
    for i in 0..size {
        for j in 0..size {
            if mat[i][j] {
                return i;
            }
        }
    }
    return 0;
}

fn last_1_row(size: usize, mat: &Vec<Vec<bool>>) -> usize {
    let mut last = 0;
    for i in 0..size {
        for j in 0..size {
            if mat[i][j] {
                last = i;
            }
        }
    }
    return last;
}

fn clear_from_point_on(x: usize, y: usize, size: usize, mat: &mut Vec<Vec<bool>>) {
    for j in y..size {
        mat[x][j] = false;
    }

    for i in (x + 1)..size {
        for j in 0..size {
            mat[i][j] = false;
        }
    }
}

fn combs ( size: usize) -> usize {
    let mut suma: usize = 1;
    let mat = &mut vec![vec![false; size]; size];
    //let mat = [[false; size];size];
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut po_nawrocie = false;

    while x < size {
        y = highest(size, &mat[x]);

        if mat[x][y] {
            if po_nawrocie {
                mat[x][y] = false;
                po_nawrocie = false;
            }
            y += 1;
        }

        while y < size && !can_put(x, y, size, mat) {
            y += 1;
        }

        if y < size {
            mat[x][y] = true;
            suma += 1;
        } else {
            if x == size - 1 {
                if already_inplace(size, mat) == 0 {
                    return suma;
                }
                x = last_1_row(size, mat);
                po_nawrocie = true;
            } else {
                x += 1;
            }
        }
    }

    return suma;
}

#[test]
fn test2() {
    assert_eq!(combs(2), 7);
}
#[test]
fn test3() {
    assert_eq!(combs(3), 63);
}
#[test]
fn test4() {
    assert_eq!(combs(4), 1234);
}
#[test]
fn test5() {
    assert_eq!(combs(5), 55447);
}
#[test]
fn test6() {
    assert_eq!(combs(6), 5598861);
}

fn main() {
    println!("Suma = {}", combs(3));
    println!("Suma = {}", combs(4));
    println!("Suma = {}", combs(5));
    println!("Suma = {}", combs(6));
    println!("Suma = {}", combs(7));
    println!("Suma = {}", combs(8));
    println!("Suma = {}", combs(9));
}
