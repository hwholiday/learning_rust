use std::fs::File;
use std::fs;
use std::io::{self, Read};
const FILE_PATH: &str="./learnerror.txt";

fn main() {
    println!("learn error");
    // fn_read_file_v2();
    // fn_read_file();
    //fn_read_file_v3();
    //fn_read_file_v4();
    test_read_file_by_path();

}

fn fn_read_file() {
    File::open(FILE_PATH).unwrap();
}

fn fn_read_file_v2() {
    File::open(FILE_PATH).expect("not find file");
}

fn fn_read_file_v3() {
    let file_info_result = File::open(FILE_PATH);
    let file_info = match file_info_result {
        Ok(file) => file,
        Err(error) => panic!("123 {}", error),
    };
}

fn fn_read_file_v4() {
    let file_info_result = File::open(FILE_PATH).unwrap_or_else(|error| {
        panic!("unwrap_or_else {}", error);
    });
}

fn test_read_file_by_path(){
   let file= match read_file_by_path(FILE_PATH){
    Ok(fs)=>fs,
    Err(error)=> panic!("read_file_by_path {}",error)
   };
   println!("read_file_by_path: {}",file);  
   
   let file= match read_file_by_path_v2(FILE_PATH){
    Ok(fs)=>fs,
    Err(error)=> panic!("read_file_by_path_v2 {}",error)
   };
   println!("read_file_by_path_v2: {}",file);
    
   let file= match read_file_by_path_v3(FILE_PATH){
    Ok(fs)=>fs,
    Err(error)=> panic!("read_file_by_path_v3 {}",error)
   };
   println!("read_file_by_path_v3: {}",file); 
   
   let file= match read_file_by_path_v4(FILE_PATH){
    Ok(fs)=>fs,
    Err(error)=> panic!("read_file_by_path_v4 {}",error)
   };
   println!("read_file_by_path_v4: {}",file);
}
fn read_file_by_path(path: &str) -> Result<String, io::Error> {
    let file_info_result = File::open(path);
    let mut file_info = match file_info_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    let mut result = String::new();
    match file_info.read_to_string(&mut result) {
        Ok(_) => Ok(result),
        Err(error) => return Err(error),
    }
}

fn read_file_by_path_v2(path:&str)->Result<String,io::Error>{
    let mut file_result=File::open(path)?;
    let mut result = String::new();
    file_result.read_to_string(&mut result)?;
    Ok(result)
}

fn read_file_by_path_v3(path:&str)->Result<String,io::Error>{
    let mut result = String::new();
    File::open(path)?.read_to_string(&mut result)?;
    Ok(result)
}

fn read_file_by_path_v4(path:&str)->Result<String,io::Error>{
   fs::read_to_string(path)
}