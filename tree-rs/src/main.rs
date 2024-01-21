use std::env;
use walkdir::{DirEntry,WalkDir};



fn is_not_hidden(entry: &DirEntry) -> bool{
    entry.file_name()
        .to_str()
        .map(|s| entry.depth()==0 || !s.starts_with("."))
        .unwrap_or(false)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut path:String = String::from(".");
    println!("*");
    if args.len() > 1{
        path = (*args[1]).to_string();
    }
    WalkDir::new(&path)
        .into_iter()
        .filter_entry(|e| is_not_hidden(e))
        .filter_map(|v|v.ok())
        .for_each(|x| println!("{}",x.path().display()));

}
