
pub enum TokenType{
    INDICATOR,
    OPERATOR,
    NUMBER,
    COMPARISON,
}


pub struct Token{

    pub t_type: TokenType,
    pub t_value: TokenValue,

}

pub enum TokenValue{

    Text(String),
    Number(f64),
}


pub fn tokenizer(str: String) -> Vec<Token>{
    
    let split = str.split_whitespace();
    let split: Vec<&str> = str.split_whitespace().collect();
    let mut vec: Vec<Token> = Vec::new();
    for idx in 0..split.len(){
        if split[idx] == "SMA" || split[idx] == "RSI"{
            let  token: Token= Token{t_type: TokenType::INDICATOR, t_value: TokenValue::Text(split[idx].to_string()),};
            vec.push(token);
        }
        else if split[idx] == "<" || split[idx] == ">"{
            let  token: Token= Token{t_type: TokenType::COMPARISON, t_value: TokenValue::Text(split[idx].to_string()),};  
            vec.push(token);
        }
        else if split[idx] == "AND" || split[idx] == "OR"{

            let  token: Token= Token{t_type: TokenType::OPERATOR, t_value: TokenValue::Text(split[idx].to_string()),};  
            vec.push(token);
        }
        else if split[idx].parse().expect("Error unknown type while parsing."){

            let  token: Token = Token{t_type: TokenType::NUMBER, t_value: TokenValue::Number(split[idx].parse().expect("Error unknown type while parsing.")),};  
            vec.push(token);
        }
        else{
            continue; 
        } 

    }

    return vec;

}


