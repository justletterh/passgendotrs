use std::env;
use rand::{self, Rng};
fn start(){
  let args: Vec<String> = env::args().collect();
  let cnt = args.len()-1;
  if cnt == 0{
    println!("no args passed, pass -h for help");
  }
  if cnt > 0{
    let arg = &args[1..args.len()];
    thing(arg);
}}
fn main(){
    start();
}
fn thing (args: &[String]){
  let mut b = args[0].chars().all(char::is_numeric);
  let mut oppt: &str = &"".to_string();
  if args.len() == 1{
    oppt = &"pass";
  }
  if args.len() != 1{
    oppt = &args[1];
  }
  let opt = oppt;
  if args[0] == "-h" || args[0] == "--help"{
    println!("passgen [len] [-u for uppercase, -s for symbols -a for all]");
  }
  else if opt != "-s" && opt != "-u"&& opt != "pass"&& opt != "-a"{
    println!("{} is not a correct arg, do -h for help", args[1]);
    b = false;
  }
  else if b == true{
    let len: i32 = args[0].parse().unwrap();
    if opt == "-a"{
      doit(len, true,true);
    }
    else if opt == "-s"{
      doit(len, true, false);
    }
    else if opt == "-u"{
      doit(len, false, true);
    }
    else {
    doit(len, false, false);
  }}
  else{
    println!("{} is not a correct argument, pass -h for help",args[0]);
}}
fn doit(len: i32, s: bool, u: bool){
  println!("{} chars long:",len);
  let mut base = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
  let mut ulet = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
  let mut sym = vec!['!', '#', '$', '%', '&', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '^', '_', '`'];
  if s == true{
    base.append(&mut sym);
  }
  if u == true{
    base.append(&mut ulet);
  }
  let mut res: String = "".to_string();
  for i in 0..len {
    res = format!("{}{}", res, base[rand::thread_rng().gen_range(0, base.len())]);
  }
  println!("{}",res);
}