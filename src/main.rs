

// use std::os;
use std::fs;
use std::env;
use std::path::Path;
// #![feature(plugin)]
// #![plugin(regex_macros)]
#[allow(unused_imports)]
use regex::Regex;

// #![feature(plugin)]
// #![plugin(regex_macros)]
extern crate regex;
// extern crate regex;
#[allow(dead_code)]
fn main() {
    // let args: ~[~str] = os::args();
    //   println(args.to_str());
    println!("{:?}", &env::current_dir().unwrap() );
    let paths = fs::read_dir(&Path::new(
      &env::current_dir().unwrap())).unwrap();
    //  for path in paths {
    //      println!("{}", path.to_str().unwrap() );
    //  }
    let _names =
        paths.map(|entry| {
          let entry = entry.unwrap();


          let entry_path = entry.path();

          let file_name = entry_path.file_name().unwrap();

          let file_name_as_str = file_name.to_str().unwrap_or("");
          println!("{:?}",file_name_as_str );
          let file_name_as_string = String::from(file_name_as_str);

          file_name_as_string
        }).collect::<Vec<String>>();

    let _re = Regex::new(r"(?x)
      (?P<a>.+) # junk
      _
      (?P<b>\d{3,}) # number // TODO : change to {1,0}
      .
      (?P<d>\pN+) # extention
    ").unwrap();

    for name in _names{
        // if name != "raven.exe"{ //rather check for not is_none
            println!("{:?}",&name );
            let _cap = _re.captures(&name);
            if !_cap.is_none(){
                println!("{}",_cap.unwrap().name("b").unwrap());
            }

        // }

    }

}
