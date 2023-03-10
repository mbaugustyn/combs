use std::u128;
use std::{time::Instant, vec};
use std::collections::{HashMap, hash_map};
use bigint::U256;

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

fn save_results  <const SIZE : usize> (result : U256, from : usize, to  : usize, mapa :  &mut HashMap<([bool; SIZE], usize), U256>, mat: &[[bool; SIZE]; SIZE]) {

    for i in from..to {
        let value = *mapa.get(&(mat[i], i)).unwrap_or(&0.into());
        let exists = value != 0.into();
        if exists {
            //print!("Zapisuje obliczony wynik dla ");
            //for i in mat[i] {print!("{}, ", i as i32 );}
            //println!(";  {} => {}", i,  result - value);
            mapa.insert((mat[i].clone(),i), result - value);   
        }
    }
}
fn combs <const SIZE: usize> () -> U256 {
    let mut suma: U256 = 1.into();
    let mut mat = [[false; SIZE];SIZE];
    let mut x: usize = 0;
    let mut y: usize;
    let mut po_nawrocie = false;
    let mut mapa = HashMap::new();


    while x < SIZE {
        y = highest::<SIZE>(mat[x]);
        if mat[x][y] {
            mat[x][y] = !po_nawrocie;
            y += 1;
        }
        while y < SIZE && !can_put::<SIZE>(x, y, &mat) {y += 1;} /* Probujemy ustawic krola */

        if y < SIZE { /* Udalo sie ustawic krola */
            mat[x][y] = true;
            suma = suma + 1.into();
            po_nawrocie = false;
        }
        else { /* Nie udalo sie ustawic krola -> albo nawrot albo probujemy w kolejnym wierszu */
            if x == SIZE - 1 { /* Nawrot  */
                if already_inplace::<SIZE>(&mat) == 0 { /* Nie ma gdzie robic nawrotu -> koniec */
                    return suma;
                }
                /* Nawrot do ostatniego krola */
                let last_row = last_1_row::<SIZE>(&mat);
                /* Zapiszmy wyniki w wierszach "na gorze" */
                save_results(suma, last_row, x, &mut mapa, &mat);
                x = last_row;
                po_nawrocie = true;
            } else { /* Probujemy w kolejnym wierszu */
                let value = *mapa.get(&(mat[x], x)).unwrap_or(&0.into());
                let exists = value != 0.into();

                if x != SIZE - 1 && exists { /* Juz to liczylismy -> nawrot */
                    suma = suma + value;
                    if already_inplace::<SIZE>(&mat) == 0 { /* Nie ma gdzie robic nawrotu -> koniec */
                        return suma;
                    }
                    let last_row = last_1_row::<SIZE>(&mat);
                    save_results(suma, last_row, x, &mut mapa, &mat);
                    x = last_row;
                    po_nawrocie = true;
                }
                else {
                    mapa.insert((mat[x].clone(), x), suma);
                    x += 1;
                    po_nawrocie = false;
                }
            }
        }
    }

    return suma;
}



#[test]
fn test2() {
    assert_eq!(combs::<2>(), 7.into());
}
#[test]
fn test3() {
    assert_eq!(combs::<3>(), 63.into());
}
#[test]
fn test4() {
    assert_eq!(combs::<4>(), 1234.into());
}
#[test]
fn test5() {
    assert_eq!(combs::<5>(), 55447.into());
}
#[test]
fn test6() {
    assert_eq!(combs::<6>(), 5598861.into());
}

#[test]
fn test7 () {
    assert_eq!(combs::<7>(), 1280128950.into());
}
#[test]
fn test8 () {
    assert_eq!(combs::<8>(), (660647962955 as u64).into());
}
#[test]
fn test9 () {
    assert_eq!(combs::<9>(), (770548397261707 as u64).into());
}
#[test]
fn test10 () {
    assert_eq!(combs::<10>(), (2030049051145980050 as u64).into());
}

fn main() {
   let start = Instant::now();

    // println!("Suma dla i = {} => {}", 2, combs::<2>());
    // println!("Suma dla i = {} => {}", 3, combs::<3>());
    // println!("Suma dla i = {} => {}", 4, combs::<4>());
    // println!("Suma dla i = {} => {}", 5, combs::<5>());
    // println!("Suma dla i = {} => {}", 6, combs::<6>());
    // println!("Suma dla i = {} => {}", 7, combs::<7>());
    // println!("Suma dla i = {} => {}", 7, combs::<7>());
    
    println!("Suma dla i = {} => {}", 12, combs::<12>());

    println!("Time elapsed is: {:?}",  start.elapsed());

    
}
