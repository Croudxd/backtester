

pub enum Node{

    Indicator { name: String, arg: Box<Node>, },
    Comparison { op: ComparisonEnum, left: Box<Node>, right: Box<Node> , },
    Operator { op: OperatorEnum, left: Box<Node>, right: Box<Node>,},
    Number (f64),
}


pub enum ComparisonEnum{
    Less,
    More,
}

pub enum OperatorEnum{
    And,
    Or,
}


/* Example:
 *
 *  SMA 10 < SMA 20 AND RSI 10 < SMA 10
 *  
 *                  AND
 *             <          <
 *         SMA   SMA   RSI  SMA
 *          N     N     N    N   
 *
 * */
