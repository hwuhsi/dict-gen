use std::{fs::File, io::Write};

use clap::Arg;



fn main() {
    let s = clap::App::new("dict-gen")
        .version("0.0.1")
        .about("dictionary generator in rust")
        .arg_from_usage("<INPUT> 'input mask'")
        .arg(Arg::from_usage("-o, --output=[FILE] 'output file'"))
        .get_matches();
    let input = s.value_of("INPUT").expect("input is required");
    let path = s.value_of("output");
    match path {
        Some(name) => {
            parse(input, name);
            //println!("{}", input);
        }
        None => {
            println!("no args!");
        }
    }
    
    
}




//few33fef?awd?d


struct Mu {
    set: &'static str,
    pos: usize
}

impl Mu {
    fn new(set: &'static str, pos: usize) -> Self {
        Mu {
            set, 
            pos
        }
    }
}



fn traverse(result: &mut String, i: usize, it: & Vec<Mu>, file: &mut File) {
    //ll.next();
    for ch in it[i].set.chars() {
        let modify = unsafe { result.as_bytes_mut() } ;
        modify[it[i].pos] = ch as u8;
        if i != 0 {
            traverse(result, i - 1, it, file);
        } else {
            //let mut file = File::create("path").unwrap();
            
            file.write_all(format!("{}\n", result).as_bytes()).unwrap();
            //println!("{}", result);
        }
    } 
}


//?l = abcdefghijklmnopqrstuvwxyz
//?u = ABCDEFGHIJKLMNOPQRSTUVWXYZ
//?d = 0123456789
//?h = 0123456789abcdef
//H = 0123456789ABCDEF
//?s = «space»!"#$%&'()*+,-./:;<=>?@[\]^_`{|}~
//?a = ?l?u?d?s
//?b = 0x00 - 0xff
static NUMBER_CHARSET: &'static str = "0123456789";
static LETTERLOWERCASE_CHARSET: &'static str = "abcdefghijklmnopqrstuvwxyz";
static LETTERCAPITALCASE_CHARSET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

static NUMBER_LCASE: &'static str = "0123456789abcdefghijklmnopqrstuvwxyz";
static NUMBER_CCASE: &'static str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static SYMBOL: &'static str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

fn parse(st: &str, filename: &str) {
    let len = st.len();

    let mut result = String::with_capacity(12);


    let mut arr: Vec<Mu> = Vec::new();


    let mut real_pos: usize = 0;

    let mut flag = false;


    for i in 0..len {
        
        if flag {
            match st.chars().nth(i).unwrap() {
                'l' => {
                    //arr.push(Me::new(CharSet::Number));
                    arr.push(Mu::new(LETTERLOWERCASE_CHARSET, real_pos - 1));
                    //result.push('?');
                    flag = false;
                }
                'u' => {
                    arr.push(Mu::new(LETTERCAPITALCASE_CHARSET, real_pos - 1));
                    //result.push('?');
                    flag = false;
                }
                'd' => {
                    arr.push(Mu::new(NUMBER_CHARSET, real_pos - 1));
                    //result.push('?');
                    flag = false;
                }
                'h' => {
                    arr.push(Mu::new(NUMBER_LCASE, real_pos - 1));
                    //result.push('?');
                    flag = false;
                }
                'H' => {
                    arr.push(Mu::new(NUMBER_CCASE, real_pos - 1));
                    //result.push('?');
                    flag = false;
                }
                's' => {
                    arr.push(Mu::new(SYMBOL, real_pos - 1));
                    //result.push('?');
                    flag = false;
                }
                _ => panic!("wrong char set!")
            }
        }else {
            
            result.push(st.chars().nth(i).unwrap());
            real_pos += 1;
            if  st.chars().nth(i) == Some('?') {
                flag = true;
                //break;
            }
            //arr.push(Me::new_with_char(s));
        }
    }
    //println!("{}", result);
    //println!("{}", arr.len());
    let mut fi = File::create(filename).expect("create file error");
    traverse(&mut result, arr.len() - 1, &arr, &mut fi);
    println!("finished\noutput file name: {}", filename)
}