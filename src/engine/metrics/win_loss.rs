use crate::engine::model::trade::Trade;

pub fn win_loss(vec: Vec<Trade>) -> f64{

        let mut wins: f64 = 0.0;
        let mut losses: f64 = 0.0;

        for idx in 0..vec.len(){
                if vec[idx].id == "buy"{
                        if idx == vec.len()-1 {
                                break;
                        }
                        let x = vec[idx + 1].price - vec[idx].price;
                        if x < 0.0{
                                losses += 1.0;
                        }
                        else if x > 0.0{
                                wins += 1.0;
                        }
               }
                else{
                        continue;
                }
        }
        return wins / losses ;
}
