use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    println!("Hello, world!");
    let path: &str = "sample.json";
    println!("file {}",path);
    let mut input :File = File::open(path).expect("No file opened!");
    let mut content :String = String::new();
    input.read_to_string(&mut content);

    let mut chars  = content.chars().peekable();
    // for c in chars{
    //     print!("{}",c)
    // }
    
    loop {
        let t =  read_token(&mut chars);
    
        if let Token::None = t{
            break
        }else{ 
            
            println!("{:?}", t);
        }
    }
    println!();
}
#[derive(Debug)]
enum Token{
    Word(String),
    StartObj,
    EndObj,
    Comma,
    StartArray,
    EndArray,
    Number(String),
    Bool(bool),
    Integer(i32),
    Float(f64),
    Colon,
    None,
}

fn read_token(chars :&mut std::iter::Peekable<std::str::Chars>)->Token{
   
    // 过滤　空白符号
    // let p = chars.peek();
    while  let Some(c) = chars.peek(){
        match c {
            ' ' | '\n' | '\r' | '\t' => chars.next(),
            _ => break,
        };
    }
     let mut word = String::new();
    let mut record =false;
    while let  Some(&c) =  chars.peek(){
           
            if record{
                 chars.next();
                if c =='"'{
                    return Token::Word(word);
                }
                word.push(c);
                continue;
            }
            if word.len()>0{
                match c{
                    ','|'}'|']' |' '|'\n'|'\r'|'\t'=>return Token::Word(word),
                    _ =>(),
                }
            }
             chars.next();
            match c{
                '{' => return Token::StartObj,
                '}'=>return Token::EndObj,
                '['=>return Token::StartArray,
                ']'=>return Token::EndArray,                
                ','=>return Token::Comma,
                ':'=> return Token::Colon,
                '"'=> if record{
                    return Token::Word(word);
                }else{
                    record = true;
                }
                '\n' => (),
                _ => //if record{
                    word.push(c),
               // }
            }
            //println!("{}",c);
        
    }
    if word.len() >0{
        if word =="true"{
            return Token::Bool(true);
        }
        if  word =="false"{
            return Token::Bool(false);
        }
        return Token::Number(word);
    }
    println!("{}", word);
    Token::None
}