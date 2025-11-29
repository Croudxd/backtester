use crate::engine::model::ast::Node;
use crate::engine::json::tokenizer; 
use crate::engine::json::tokenizer::Token;
use crate::engine::json::tokenizer::TokenType;
use crate::engine::json::tokenizer::TokenValue;
use crate::engine::model::ast::ComparisonEnum;
use crate::engine::model::ast::OperatorEnum;



pub fn parser(vec: &mut Vec<Token>){
    parse_operator(vec);
}


pub fn parse_term(tokens: &mut Vec<Token>) -> Node{
    let token = tokens.remove(0);
    match token.t_type{
        TokenType::INDICATOR => {
            let name = match token.t_value {
                TokenValue::Text(s) => s,
                _ => panic!("Indicator must have text value"),
            };

            if tokens.is_empty(){
                panic!("Expected number after indicator declaration.")
            }
        
            let next = tokens.remove(0);
            
            match (next.t_type, next.t_value) {
                    (TokenType::NUMBER, TokenValue::Number(n)) => {
                        Node::Indicator {
                            name,
                            arg: Box::new(Node::Number(n)),
                        }
                    }
                    _ => panic!("Expected number literal after indicator"),
                
            }
        }
        TokenType::NUMBER => match token.t_value {
            TokenValue::Number(n) => Node::Number(n),
            _ => panic!("TokenType::NUMBER but value was not Number"),
        },

        _ => panic!("Unexpected token in term"),

        
    }
}


pub fn parse_comparison(tokens: &mut Vec<Token>) -> Node{
    let left = parse_term(tokens);
    if tokens.is_empty() {
        return left; 
    }

    let token = tokens.remove(0);


    match token.t_type {
        TokenType::COMPARISON => {
            let op_str = match token.t_value {
                TokenValue::Text(s) => s,
                _ => panic!("Comparison operator must be text (like < or >)"),
            };

            let op = match op_str.as_str() {
                "<" => ComparisonEnum::Less,
                ">" => ComparisonEnum::More,
                _ => panic!("Unknown comparison operator '{}'", op_str),
            };

            let right = parse_term(tokens);

            Node::Comparison {
                op,
                left: Box::new(left),
                right: Box::new(right),
            }
        }

        _ => left,
    }

}


pub fn parse_operator(tokens: &mut Vec<Token>) -> Node{
    let left = parse_comparison(tokens);
    if tokens.is_empty(){
        return left;
    }
    let token = tokens.remove(0);
    match token.t_type {
        TokenType::OPERATOR => {
            let op_str = match token.t_value {
                TokenValue::Text(s) => s,
                _ => panic!("Operator statement must be text (like AND or OR)"),
            };

            let op = match op_str.as_str() {
                "AND" => OperatorEnum::And,
                "OR" => OperatorEnum::Or,
                _ => panic!("Unknown comparison operator '{}'", op_str),
            };

            let right = parse_term(tokens);

            Node::Operator {
                op,
                left: Box::new(left),
                right: Box::new(right),
            }
        }

        _ => left,
    }

}

