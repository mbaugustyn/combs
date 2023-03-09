use std::{time::Instant, vec};
use std::collections::{HashMap, hash_map};

fn can_put <const SIZE: usize> (x: usize, y: usize, mat: &[[bool; SIZE]; SIZE]) -> bool {
    if y > 0 && mat[x][y - 1] {
        return false;
    }
    if x > 0 && mat[x - 1][y] { 
        return false;
    }
    return true;
}

fn print_mat <const SIZE: usize> (mat: &[[bool; SIZE]; SIZE]) {
    println!("\nMat:");
    for i in 0..SIZE {
        for j in 0..SIZE {
            print!("{} ", mat[i][j] as i8);
        }
        println!();
    }
    println!();
}

fn highest <const SIZE: usize>(row: [bool; SIZE]) -> usize {
    let mut max = 0;
    for i in 0..SIZE {
        if row[i] == true {
            max = i;
        }
    }
    return max;
}


fn already_inplace <const SIZE : usize> (mat: &[[bool; SIZE]; SIZE]) -> usize {
    let mut suma = 0;
    for i in 0..SIZE {
        for j in 0..SIZE {
            suma += mat[i][j] as usize;
        }
    }
    return suma;
}



fn last_1_row <const SIZE : usize> (mat: &[[bool; SIZE]; SIZE]) -> usize {
    let mut last = 0;
    for i in 0..SIZE {
        for j in 0..SIZE {
            if mat[i][j] {
                last = i;
            }
        }
    }
    return last;
}



//Backtracking might be too slow. One can improve its speed using memoization taking into account that the number of solutions at a certain row k only depends on the position of the king in row k-1 and the set of occupied positions for rows <k.

fn print_hash <const SIZE : usize> (mapa :  &HashMap<([bool; SIZE], usize), usize>) {
    println!("\nMapa:");
    for ((v,cnt), value) in mapa.iter() {
        for i in v {print!("{}, ", *i as i32 );}
        print!(" ; {}", cnt);
        print!(" => {}", value);
        println!("");
    }
}

fn save_results  <const SIZE : usize> (result : usize, from : usize, to  : usize, mapa :  &mut HashMap<([bool; SIZE], usize), usize>, mat: &[[bool; SIZE]; SIZE]) {
    for i in from..to {
        //let row = mat[i];
        let value = mapa.get(&(mat[i], i)).unwrap_or(&0);
        let exists = value != &0;
        if exists {
            mapa.insert((mat[i].clone(),i), result - value + 1);   
        }
        else {
            println!("COS NIE TAK");
        }
    }
}
fn combs <const SIZE: usize> () -> usize {
    let mut suma: usize = 1;
    let mut mat = [[false; SIZE];SIZE];
    let mut x: usize = 0;
    let mut y: usize;
    let mut po_nawrocie = false;
    let mut mapa = HashMap::new();


    while x < SIZE {

        y = highest::<SIZE>(mat[x]);
        print_hash(&mapa);

        if mat[x][y] {
            if po_nawrocie {
                mat[x][y] = false;
                //po_nawrocie = false;
            }
            y += 1;
        }

        while y < SIZE && !can_put::<SIZE>(x, y, &mat) {y += 1;} /* Probujemy ustawic krola */

        if y < SIZE { /* Udalo sie ustawic krola */
            mat[x][y] = true;
            print_mat::<SIZE>(&mat);
            suma += 1;
            println!("1. Nowa suma = {}", suma);
            po_nawrocie = false;
        } else { /* Nie udalo sie ustawic krola -> albo nawrot albo probujemy w kolejnym wierszu */
            if x == SIZE - 1 { /* Nawrot  */
                if already_inplace::<SIZE>(&mat) == 0 { /* Nie ma gdzie robic nawrotu -> koniec */
                    return suma;
                }
                /* Nawrot do ostatniego krola */
                let last_row = last_1_row::<SIZE>(&mat);
                /* Zapiszmy w tym wierszu ilu sie kombinacji doliczylismy dla niego */
                if x != last_row {
                    let value = mapa.get(&(mat[last_row], last_row)).unwrap_or(&0);
                    let exists = value != &0;
                    if exists {
                        println!("Zapisuje obliczony wynik dla ({:?} {}) , wynik => {}",
                             mat[last_row] , last_row,  suma - value);
                        save_results(suma, last_row, x, &mut mapa, &mat);
                    }
                    else {println!("COS NIE TAK");}
                }
                x = last_row;
                po_nawrocie = true;
            } else { /* Probujemy w kolejnym wierszu */
                let value = mapa.get(&(mat[x], x)).unwrap_or(&0);
                let exists = value != &0;
                if exists { /* Juz to liczylismy -> nawrot */
                    println!("Juz to liczylismy ({:?}, {})  = {} -> nawrot", mat[x], x, value);
                    if !po_nawrocie {suma += value;}
                    println!("2. Nowa suma = {}", suma);
                    x = last_1_row::<SIZE>(&mat);
                    po_nawrocie = true;
                    if already_inplace::<SIZE>(&mat) == 0 { /* Nie ma gdzie robic nawrotu -> koniec */
                        return suma;
                    }
                    
                }
                else { /* Jeszcze tego nie liczylismy */
                    println!("Jeszcze tego nie liczylismy");
                    mapa.insert((mat[x].clone(), x), suma);
                    x += 1;
                }
            }
        }
    }

    return suma;
}



#[test]
fn test2() {
    assert_eq!(combs::<2>(), 7);
}
#[test]
fn test3() {
    assert_eq!(combs::<3>(), 63);
}
#[test]
fn test4() {
    assert_eq!(combs::<4>(), 1234);
}
#[test]
fn test5() {
    assert_eq!(combs::<5>(), 55447);
}
// #[test]
// fn test6() {
//     assert_eq!(combs::<6>(), 5598861);
// }

// #[test]
// fn test7 () {
//     assert_eq!(combs::<7>(), 1 280 128 950);
// }

fn main() {
//    let start = Instant::now();
    println!("Suma dla i = {} => {}", 3, combs::<3>());
    //println!("Time elapsed is: {:?}",  start.elapsed());

    
}
