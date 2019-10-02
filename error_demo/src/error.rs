use std::io::{self, Read, ErrorKind};
use std::fs::File;

pub fn file_error() {
    let f = File::open("hello.txt");   
    let f = match f {
        Ok(f) => f,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(f) => f,
                Err(err) => panic!("fail to create file.{}", err),
            },
            other_error =>panic!("fail to open file.{:#?} {}", error.kind(), error)
        },
    };
    println!("f is {:#?}", f);

    //let f = File::open("hello2.txt").unwrap();
    let f = File::open("hello2.txt").expect("fail to open hello2.txt");
}


pub fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
