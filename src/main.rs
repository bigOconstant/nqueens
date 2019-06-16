fn main() {
    println!("");
    let mut vec:Vec<Vec<i32>> = Vec::new();
    for _i in 0..4 {vec.push(vec![0,0,0,0])};
    
    
    nqueens(&mut vec,0);
    attacked(&vec,2,3);
   

    println!("{:?}",vec);

}
/*
    ToDo Check if peice is attacked
*/
fn attacked(input:&Vec<Vec<i32>>,row:i32,col:i32)-> bool {
   

    for i in 0..input.len() {

        for j in 0..input.len() {
            if input[i][j] == 1 {
                if i == col as usize || j == row as usize {
                    return true
                }
                if (row - col) == (j as i32) - (i as i32) || (row + col) == ((j as i32)+(i as i32)) {
                    return true
                }
            }
        }
        
    }

    return false;
}

fn nqueens( input:&mut Vec<Vec<i32>>,col:i32)-> bool{
    if col == input.len() as i32 {
        return true;
    }
    input[0][col as usize] = 8;
     return nqueens(input,col+1);


   
}
