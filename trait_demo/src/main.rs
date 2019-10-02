mod trait_demo;
use trait_demo::Summary;

fn main() {
    let v = vec![1,2,3,4,5];
    let l = trait_demo::largest(&v);
    println!("largest value is {}", l);

    let article = trait_demo::NewsArticle{
        headline:String::from("hf"), 
        location:String::from("fad"),
        author:String::from("SAd"),
        content:String::from("jkl;")};
    println!("summary is {}", article.summarize());
    trait_demo::notify(&article);

}
