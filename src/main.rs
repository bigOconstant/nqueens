/*
Find the number of n queen configurations given a board size
*/

fn main() {
    println!("finding n queens");

    //Create a board (vector of vectors)
    let mut board:Vec<Vec<i32>> = Vec::new();

    //Set the size of the board and the number of queens
    let sizeofvec = 10;

    // initialize board with zeros to represent empty spaces
    for _i in 0..sizeofvec {
        let mut minivec = Vec::new();
        for _j in 0..sizeofvec {
            minivec.push(0);
        }
        board.push(minivec);
    };
    
    let mut found = 0;
    nqueens(&mut board,0,&mut found);
    println!("found: {} nqueen combinations",found);
}

//Recursive nqueen function to find the n queens
fn nqueens( input:&mut Vec<Vec<i32>>,row:i32, found:&mut i32)-> bool{
    if row == (input.len() + 1) as i32 {
        return false;
    }
    for i in 0..input.len() {
        let attackedme = attacked(&input,row,i as i32);
        
        if !attackedme{
            input[row as usize][i as usize] = 1;
            if !nqueens(input,row+1,found){
                input[row as usize][i as usize] = 0;
            }
        }
    }

    if countqueens(input) == input.len() as i32 {
        *found = *found +1;
       // printqueens(input);
    }
    false
}

/*
     Checks if piece can be attacked in this possition
*/
fn attacked(input:&Vec<Vec<i32>>,row:i32,col:i32)-> bool {
    for i in 0..input.len() {
        for j in 0..input.len() {
            if input[i][j] == 1 {
                if i == row as usize || j == col as usize {
                    return true
                }
                if (row - col) == (i as i32) - (j as i32) || (row + col) == ((j as i32)+(i as i32)) {
                    return true
                }
            }
        }
    }
    return false;
}

// Prints the board if you want to see it
fn printqueens(input:&Vec<Vec<i32>>){
    println!("  ");
    for i in input {
        println!("{:?}",i);
    }
    
}

// Counts the number of queens on the board
fn countqueens(input:&Vec<Vec<i32>>)-> i32 {
    let mut count = 0;
    for i in 0..input.len() {
        for j in 0..input.len() {
            if input[i][j] == 1 {
                count = count +1;
            }
        }
    }
    count
}
