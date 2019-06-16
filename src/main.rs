fn main() {
    println!("finding n queens");
    let mut vec:Vec<Vec<i32>> = Vec::new();
    let sizeofvec = 10;
    for _i in 0..sizeofvec {
        let mut minivec = Vec::new();
        for j in 0..sizeofvec {
            minivec.push(0);
        }
        vec.push(minivec);
    };
    
    let mut found = 0;
    nqueens(&mut vec,0,&mut found);

    println!("found: {} nqueen combinations",found);
   



}

fn printqueens(input:&Vec<Vec<i32>>){
    println!("  ");
    for i in input {
        println!("{:?}",i);
    }
    
}

/*
     Checks if peice is attacked
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
