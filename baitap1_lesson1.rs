fn main() {
    let array1:[i32;8] = [1,2,3,5,6,8,10,11];
    let array2:[i32;3] = [6,8,10];

    println!("Array1 : {:?}", array1);
    println!("Array2 : {:?}", array2);

    if(isSubset(&array1,&array2)){
        println!("Array2 is a subset of Array1");
    }else{ 
        println!("Array2 is not a subset of Array1");
    }

}

fn isSubset(arr1:&[i32], arr2:&[i32]) -> bool{
    let mut i = 0;
    let mut j = 0;
    let mut count = 0;

    for i in 0..arr2.len(){
        for j in 0..arr1.len(){
            if(arr2[i] == arr1[j]){
                count+=1;
            }
        }
    }

    if(count != arr2.len()){
        return false;
    }
    return true;

}
