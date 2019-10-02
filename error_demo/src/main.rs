mod error;

fn main() {
    let vec = vec![1,2,3,4,5];
    //error::file_error();
    let r = error::read_username_from_file();
    println!("return {:#?}", r);
}
