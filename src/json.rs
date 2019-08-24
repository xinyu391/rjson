
use std::collections::HashMap;
use std::ops::Index;


// pub mod rjson{

#[derive(Debug)]
pub enum Value{
    Bool(bool),
    String(String),
    Integer(i32),
    Float(f64),
    Array(Vec<Value>),
    Object(Object),
    Null,
}
#[derive(Debug)]
pub struct Object{
    items : HashMap<String,Value>,
}
impl Index<&str> for Object{
    type Output = Value;
    fn index(&self, key :&str)->&Value{
        &self.items[key]
    }
}
impl Object{
    fn new()->Object{
        Object{items:HashMap::new()}
    }
    pub fn get(& self, key : &str)->&Value{
        &self.items[key]
    }
}
pub fn parse(content :&String)->Object{
    let mut chars = content.chars().peekable();
    let t = read_token(&mut chars);
    if let Token::StartObj = t{
        let obj =  read_object(&mut chars);
        return obj;
    }
    let obj =  Object::new();
    obj 
}
fn read_object(chars: &mut std::iter::Peekable<std::str::Chars>)->Object{
    let mut obj = Object::new();
    loop {
        let t = read_token(chars);
        if let Token::None = t {
            break;
        } else {
            println!("{:?}", t);

            if let Token::EndObj=t{
                break;
            }
            if let Token::String(s)=t{
                // key s
                let key = s;
                let t = read_token(chars);
                if let Token::Colon =t {
                    let t = read_token(chars);
                    // value t
                    println!("{}:{:?}",key,t);
                    // obj, value, array,
                    match t{
                        Token::StartObj =>{
                            let obj_ = read_object(chars);
                            obj.items.insert(key,Value::Object(obj_));
                        }
                        Token::StartArray => {
                            let array = read_array(chars);
                           obj.items.insert(key,Value::Array(array));
                        }
                        Token::Integer(n)=>{
                            obj.items.insert(key, Value::Integer(n));
                        }
                        Token::Float(n)=>{
                            obj.items.insert(key, Value::Float(n));
                        }
                        Token::Bool(b)=>{
                            obj.items.insert(key, Value::Bool(b));
                        }
                        Token::String(s) =>{
                            obj.items.insert(key, Value::String(s));
                        }
                        Token::Null =>{
                            obj.items.insert(key, Value::Null);
                        }
                        _ => (),
                    }   
                }
            }
        }
    }
    obj
}
fn read_array(chars: &mut std::iter::Peekable<std::str::Chars>)->Vec<Value>{
    let mut array : Vec<Value>= Vec::new();
    loop{
        let t = read_token(chars);
        match t{
            Token::Comma =>(),
            Token::String(s) => array.push(Value::String(s)),
            Token::Integer(n)=>array.push(Value::Integer(n)),
            Token::Float(n)=>array.push(Value::Float(n)),
            Token::Bool(b)=>array.push(Value::Bool(b)),
            Token::StartArray =>{
                let _array = read_array(chars);
                array.push(Value::Array(_array));
            }
            Token::StartObj =>{
                let obj = read_object(chars);
                array.push(Value::Object(obj));
            }
            Token::EndArray =>{
                break;
            }
            _ =>(),
        }
    }
    array
}
#[derive(Debug)]
enum Token {
    String(String),
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
    Null,
    // Word(String), // true/false/null/numbers
    None,
}

fn read_token(chars: &mut std::iter::Peekable<std::str::Chars>) -> Token {
    // 过滤　空白符号
    // let p = chars.peek();
    while let Some(c) = chars.peek() {
        match c {
            ' ' | '\n' | '\r' | '\t' => chars.next(),
            _ => break,
        };
    }
    let mut word = String::new();
    let mut record = false;
    while let Some(&c) = chars.peek() {
        if record {
            chars.next();
            if c == '"' {
                return Token::String(word);
            }
            word.push(c);
            continue;
        }
        if word.len() > 0 {
            match c {
                ',' | '}' | ']' | ' ' | '\n' | '\r' | '\t' => {
                    if word == "true" {
                        return Token::Bool(true);
                    }
                    if word == "false" {
                        return Token::Bool(false);
                    }
                    if word == "null" {
                        return Token::Null;
                    }
                    if let Some(_i) = word.find('.'){
                        let f :f64 = word.parse().unwrap();
                        return Token::Float(f);
                    }else{
                        let n :i32 = word.parse().unwrap();
                        return Token::Integer(n);
                    }
                }
                _ => (),
            }
        }
        chars.next();
        match c {
            '{' => return Token::StartObj,
            '}' => return Token::EndObj,
            '[' => return Token::StartArray,
            ']' => return Token::EndArray,
            ',' => return Token::Comma,
            ':' => return Token::Colon,
            '"' => {
                if record {
                    return Token::String(word);
                } else {
                    record = true;
                }
            }
            '\n' => (),
            _ =>
            //if record{
            {
                word.push(c)
            } // }
        }
        //println!("{}",c);
    }
    if word.len() > 0 {
        if word == "true" {
            return Token::Bool(true);
        }
        if word == "false" {
            return Token::Bool(false);
        }
        return Token::Number(word);
    }
    println!("{}", word);
    Token::None
}

// }