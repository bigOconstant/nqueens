fn main() {
    println!("");
    let mut vec:Vec<Vec<i32>> = Vec::new();
    for i in 0..4 {vec.push(vec![0,0,0,0])};
    
    
    for i in 0 .. vec.len(){
        println!("{:?}",vec[i]);
       // vec[i][2] = 3;
    }
    
    nqueens(&mut vec,0);
    
   

    println!("{:?}",vec);

}
/*
    ToDo Check if peice is attacked
*/
fn attacked()->bool{
    return true;
}

fn nqueens( input:&mut Vec<Vec<i32>>,mut col:i32)-> bool{
    if col == input.len() as i32 {
        return true;
    }
    input[0][col as usize] = 8;
     return nqueens(input,col+1);


   
}
